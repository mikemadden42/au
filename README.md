# au

Search for gold in text files: a fast, parallel scanner for leaked secrets (and, optionally, personal data).

NOTE: This tool should be used for authorized testing and/or educational purposes only.

## Install

```sh
cargo install --path .
```

## Usage

```sh
au                      # scan the current directory for secrets
au src config/.env      # scan specific directories or files
au --pii                # also report emails, phone numbers, SSNs, card numbers, IPs
au --redact             # hide the matched text in the output
au --list-patterns      # show every pattern and its category
au --only jwt,private-key
au --skip secret-assignment
au --all                # scan every non-binary file, not just known extensions
au --ext env,json,yaml  # choose which extensions to scan
```

Each finding prints once per line, with the names of the patterns that matched:

```
./deploy/.env:3: [secret-assignment] DB_PASSWORD=hunter22
./notes.txt:12: [aws-access-key-id] AKIAIOSFODNN7EXAMPLE
```

Findings are sorted by path and line.

### Exit status

| Code | Meaning |
|------|---------|
| 0 | Nothing found |
| 1 | Findings reported |
| 2 | An error occurred (bad arguments, unreadable path) and nothing was found |

### What gets scanned

- Directories are walked recursively in parallel. Symlinks are not followed.
- `.gitignore`, `.ignore` and global git excludes are honored; pass `--no-ignore` to scan everything.
- Hidden files such as `.env` are scanned. `.git` directories are skipped.
- By default only common text, config and source extensions are scanned (`au --help` lists them). Dotfiles match by name, so `.env` and `.env.local` count as `env`.
- Files named on the command line are always scanned, whatever their extension.
- Binary files (a NUL byte in the first 8 KiB) and files over 10 MiB (`--max-filesize`) are skipped.
- Invalid UTF-8 is decoded lossily rather than skipped.

### Patterns

| Name | Category |
|------|----------|
| `aws-access-key-id` | secret |
| `aws-secret-access-key` | secret |
| `private-key` | secret |
| `jwt` | secret |
| `bearer-token` | secret |
| `github-token` | secret |
| `slack-token` | secret |
| `secret-assignment` | secret |
| `url-credentials` | secret |
| `email` | pii |
| `us-ssn` | pii |
| `us-phone` | pii |
| `credit-card` | pii |
| `ipv4` | pii |
| `ipv6` | pii |

Patterns live in [`src/patterns.rs`](src/patterns.rs), each with positive and negative tests.

## History

This started as an experiment in asking ChatGPT to help write a Rust tool. The original conversation is kept in [`docs/chatgpt-transcript.md`](docs/chatgpt-transcript.md).
