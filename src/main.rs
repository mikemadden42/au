mod patterns;

use clap::Parser;
use ignore::{WalkBuilder, WalkState};
use patterns::{Category, PATTERNS};
use regex::{Regex, RegexSet};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

const DEFAULT_EXTENSIONS: &[&str] = &[
    "txt",
    "csv",
    "log",
    "md",
    "env",
    "json",
    "yaml",
    "yml",
    "toml",
    "ini",
    "cfg",
    "conf",
    "properties",
    "xml",
    "sql",
    "tf",
    "tfvars",
    "sh",
    "bash",
    "zsh",
    "ps1",
    "py",
    "js",
    "ts",
    "rb",
    "go",
    "rs",
    "java",
    "php",
];

/// Bytes inspected for a NUL to decide a file is binary.
const BINARY_SNIFF_LEN: usize = 8192;

/// Search for gold (secrets and personal data) in text files.
///
/// Exits 0 when nothing is found, 1 when there are findings, and 2 on errors.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Files or directories to scan. Files named explicitly are scanned whatever their extension.
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// File extensions to scan, comma-separated. A dotfile such as `.env` or `.env.local` matches `env`.
    #[arg(short, long, value_delimiter = ',', default_values = DEFAULT_EXTENSIONS)]
    ext: Vec<String>,

    /// Scan every file regardless of extension. Binary files are still skipped.
    #[arg(short, long)]
    all: bool,

    /// Also report personal data: emails, phone numbers, SSNs, card numbers and IP addresses.
    #[arg(long)]
    pii: bool,

    /// Run only these patterns, comma-separated (see --list-patterns). Overrides --pii.
    #[arg(long, value_delimiter = ',', value_name = "NAMES")]
    only: Vec<String>,

    /// Skip these patterns, comma-separated.
    #[arg(long, value_delimiter = ',', value_name = "NAMES")]
    skip: Vec<String>,

    /// List the available patterns and exit.
    #[arg(long)]
    list_patterns: bool,

    /// Replace matched text with [REDACTED] in the output.
    #[arg(short, long)]
    redact: bool,

    /// Don't honor .gitignore, .ignore and similar files.
    #[arg(long)]
    no_ignore: bool,

    /// Skip files larger than this many bytes.
    #[arg(long, default_value_t = 10 * 1024 * 1024, value_name = "BYTES")]
    max_filesize: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Finding {
    path: PathBuf,
    line: usize,
    patterns: Vec<&'static str>,
    text: String,
}

struct Scanner {
    set: RegexSet,
    regexes: Vec<Regex>,
    names: Vec<&'static str>,
    redact: bool,
}

impl Scanner {
    fn new(indices: &[usize], redact: bool) -> Self {
        Scanner {
            set: RegexSet::new(indices.iter().map(|&i| PATTERNS[i].regex))
                .expect("built-in patterns must compile"),
            regexes: patterns::compile(indices),
            names: indices.iter().map(|&i| PATTERNS[i].name).collect(),
            redact,
        }
    }

    fn scan_file(&self, path: &Path) -> io::Result<Vec<Finding>> {
        let bytes = fs::read(path)?;
        if bytes[..bytes.len().min(BINARY_SNIFF_LEN)].contains(&0) {
            return Ok(Vec::new());
        }
        // Lossy decoding keeps lines with invalid UTF-8 instead of silently dropping them.
        let text = String::from_utf8_lossy(&bytes);
        Ok(self.scan_text(&text, path))
    }

    fn scan_text(&self, text: &str, path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        for (index, line) in text.lines().enumerate() {
            let matched: Vec<usize> = self.set.matches(line).into_iter().collect();
            if matched.is_empty() {
                continue;
            }
            findings.push(Finding {
                path: path.to_path_buf(),
                line: index + 1,
                patterns: matched.iter().map(|&i| self.names[i]).collect(),
                text: if self.redact {
                    self.redact_line(line, &matched)
                } else {
                    line.to_string()
                },
            });
        }
        findings
    }

    fn redact_line(&self, line: &str, matched: &[usize]) -> String {
        let mut spans: Vec<(usize, usize)> = matched
            .iter()
            .flat_map(|&i| {
                self.regexes[i]
                    .find_iter(line)
                    .map(|m| (m.start(), m.end()))
            })
            .collect();
        spans.sort_unstable();

        let mut out = String::with_capacity(line.len());
        let mut pos = 0;
        for (start, end) in spans {
            if end <= pos {
                continue;
            }
            if start >= pos {
                out.push_str(&line[pos..start]);
                out.push_str("[REDACTED]");
            }
            // An overlapping span extends the redaction already written.
            pos = end;
        }
        out.push_str(&line[pos..]);
        out
    }
}

/// Picks the indices into `PATTERNS` to run.
fn select_patterns(args: &Args) -> Result<Vec<usize>, String> {
    let index_of = |name: &str| {
        PATTERNS
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| format!("unknown pattern {name:?} (see --list-patterns)"))
    };

    let mut selected: Vec<usize> = if args.only.is_empty() {
        PATTERNS
            .iter()
            .enumerate()
            .filter(|(_, p)| p.category == Category::Secret || args.pii)
            .map(|(i, _)| i)
            .collect()
    } else {
        args.only
            .iter()
            .map(|n| index_of(n))
            .collect::<Result<_, _>>()?
    };
    let skipped: Vec<usize> = args
        .skip
        .iter()
        .map(|n| index_of(n))
        .collect::<Result<_, _>>()?;

    selected.sort_unstable();
    selected.dedup();
    selected.retain(|i| !skipped.contains(i));
    if selected.is_empty() {
        return Err("no patterns selected".to_string());
    }
    Ok(selected)
}

fn has_wanted_extension(path: &Path, exts: &[String]) -> bool {
    let wanted = |s: &OsStr| exts.iter().any(|e| s.eq_ignore_ascii_case(e));
    if path.extension().is_some_and(wanted) {
        return true;
    }
    // Dotfiles: `.env` and `.env.local` both count as `env`.
    path.file_name()
        .and_then(OsStr::to_str)
        .and_then(|name| name.strip_prefix('.'))
        .and_then(|name| name.split('.').next())
        .is_some_and(|stem| wanted(OsStr::new(stem)))
}

/// Walks `args.paths` in parallel. Returns sorted findings and whether any error occurred.
fn scan(args: &Args, scanner: &Scanner) -> (Vec<Finding>, bool) {
    let mut builder = WalkBuilder::new(&args.paths[0]);
    for path in &args.paths[1..] {
        builder.add(path);
    }
    builder
        .standard_filters(!args.no_ignore)
        // Hidden files such as `.env` are prime targets; only skip VCS metadata.
        .hidden(false)
        .filter_entry(|entry| entry.file_name() != ".git")
        .follow_links(false)
        .max_filesize(Some(args.max_filesize));

    let findings = Mutex::new(Vec::new());
    let had_error = AtomicBool::new(false);

    builder.build_parallel().run(|| {
        Box::new(|result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("au: {err}");
                    had_error.store(true, Ordering::Relaxed);
                    return WalkState::Continue;
                }
            };
            if !entry.file_type().is_some_and(|t| t.is_file()) {
                return WalkState::Continue;
            }
            let path = entry.path();
            let explicit = entry.depth() == 0;
            if !explicit && !args.all && !has_wanted_extension(path, &args.ext) {
                return WalkState::Continue;
            }
            match scanner.scan_file(path) {
                Ok(found) if !found.is_empty() => findings.lock().unwrap().extend(found),
                Ok(_) => {}
                Err(err) => {
                    eprintln!("au: {}: {err}", path.display());
                    had_error.store(true, Ordering::Relaxed);
                }
            }
            WalkState::Continue
        })
    });

    let mut findings = findings.into_inner().unwrap();
    findings.sort();
    (findings, had_error.into_inner())
}

fn list_patterns() {
    for p in PATTERNS {
        let category = match p.category {
            Category::Secret => "secret",
            Category::Pii => "pii",
        };
        println!("{:<22} {:<7} {}", p.name, category, p.description);
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    if args.list_patterns {
        list_patterns();
        return ExitCode::SUCCESS;
    }
    let indices = match select_patterns(&args) {
        Ok(indices) => indices,
        Err(err) => {
            eprintln!("au: {err}");
            return ExitCode::from(2);
        }
    };

    let scanner = Scanner::new(&indices, args.redact);
    let (findings, had_error) = scan(&args, &scanner);
    for f in &findings {
        println!(
            "{}:{}: [{}] {}",
            f.path.display(),
            f.line,
            f.patterns.join(", "),
            f.text
        );
    }

    if !findings.is_empty() {
        ExitCode::from(1)
    } else if had_error {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(argv: &[&str]) -> Args {
        Args::parse_from(std::iter::once("au").chain(argv.iter().copied()))
    }

    fn scanner(argv: &[&str]) -> Scanner {
        let a = args(argv);
        Scanner::new(&select_patterns(&a).unwrap(), a.redact)
    }

    fn names(indices: &[usize]) -> Vec<&'static str> {
        indices.iter().map(|&i| PATTERNS[i].name).collect()
    }

    #[test]
    fn default_selection_is_secrets_only() {
        let selected = select_patterns(&args(&[])).unwrap();
        assert!(
            selected
                .iter()
                .all(|&i| PATTERNS[i].category == Category::Secret)
        );
        let with_pii = select_patterns(&args(&["--pii"])).unwrap();
        assert_eq!(with_pii.len(), PATTERNS.len());
    }

    #[test]
    fn only_and_skip() {
        let selected = select_patterns(&args(&["--only", "email,jwt", "--skip", "jwt"])).unwrap();
        assert_eq!(names(&selected), ["email"]);
        assert!(select_patterns(&args(&["--only", "nope"])).is_err());
        assert!(select_patterns(&args(&["--only", "jwt", "--skip", "jwt"])).is_err());
    }

    #[test]
    fn extension_filter() {
        let exts = args(&[]).ext;
        assert!(has_wanted_extension(Path::new("a/notes.TXT"), &exts));
        assert!(has_wanted_extension(Path::new("a/.env"), &exts));
        assert!(has_wanted_extension(Path::new("a/.env.local"), &exts));
        assert!(has_wanted_extension(Path::new("config.yaml"), &exts));
        assert!(!has_wanted_extension(Path::new("image.png"), &exts));
        assert!(!has_wanted_extension(Path::new("Makefile"), &exts));
    }

    #[test]
    fn one_finding_per_line_listing_every_pattern() {
        let s = scanner(&["--pii"]);
        let found = s.scan_text(
            "nothing here\nThe meeting is at 1030\nusername\nuser jane@example.com key AKIAIOSFODNN7EXAMPLE\n",
            Path::new("f.txt"),
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].line, 4);
        assert_eq!(found[0].patterns, ["aws-access-key-id", "email"]);
    }

    #[test]
    fn redaction() {
        let s = scanner(&["--redact", "--pii"]);
        let found = s.scan_text("a jane@example.com b password=hunter22 c", Path::new("f"));
        assert_eq!(found[0].text, "a [REDACTED] b [REDACTED] c");

        // Overlapping matches collapse into one redaction.
        let found = s.scan_text("x password=jane@example.com y", Path::new("f"));
        assert_eq!(found[0].text, "x [REDACTED] y");
    }

    #[test]
    fn scans_directory_tree() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("sub/deeper")).unwrap();
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".env"), "API_KEY=abcd1234efgh\n").unwrap();
        fs::write(
            root.join("sub/deeper/notes.txt"),
            "ok\nok\nAKIAIOSFODNN7EXAMPLE\n",
        )
        .unwrap();
        fs::write(root.join("sub/clean.txt"), "nothing 1030 username\n").unwrap();
        fs::write(root.join("sub/skipped.png"), "password=hunter22\n").unwrap();
        fs::write(root.join("sub/bin.txt"), b"password=hunter22\0\x01").unwrap();
        fs::write(root.join("sub/latin1.txt"), b"caf\xe9 password=hunter22\n").unwrap();
        fs::write(root.join(".git/config"), "password=hunter22\n").unwrap();

        let a = args(&[root.to_str().unwrap()]);
        let s = Scanner::new(&select_patterns(&a).unwrap(), false);
        let (found, had_error) = scan(&a, &s);
        assert!(!had_error);
        let got: Vec<_> = found
            .iter()
            .map(|f| (f.path.strip_prefix(root).unwrap().to_path_buf(), f.line))
            .collect();
        assert_eq!(
            got,
            [
                (PathBuf::from(".env"), 1),
                (PathBuf::from("sub/deeper/notes.txt"), 3),
                (PathBuf::from("sub/latin1.txt"), 1),
            ]
        );

        // `--all` picks up the .png; naming a file explicitly does too.
        let a = args(&["--all", root.to_str().unwrap()]);
        assert_eq!(scan(&a, &s).0.len(), 4);
        let a = args(&[root.join("sub/skipped.png").to_str().unwrap()]);
        assert_eq!(scan(&a, &s).0.len(), 1);
    }

    #[test]
    fn missing_path_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let a = args(&[dir.path().join("missing").to_str().unwrap()]);
        let s = Scanner::new(&select_patterns(&a).unwrap(), false);
        let (found, had_error) = scan(&a, &s);
        assert!(found.is_empty());
        assert!(had_error);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_loop_terminates() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("a")).unwrap();
        fs::write(dir.path().join("a/x.txt"), "password=hunter22\n").unwrap();
        std::os::unix::fs::symlink(dir.path(), dir.path().join("a/loop")).unwrap();

        let a = args(&[dir.path().to_str().unwrap()]);
        let s = Scanner::new(&select_patterns(&a).unwrap(), false);
        assert_eq!(scan(&a, &s).0.len(), 1);
    }
}
