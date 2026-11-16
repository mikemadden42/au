use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    /// Credentials and keys. Enabled by default.
    Secret,
    /// Personal data. Enabled with `--pii`.
    Pii,
}

pub struct Pattern {
    pub name: &'static str,
    pub category: Category,
    pub description: &'static str,
    pub regex: &'static str,
}

pub const PATTERNS: &[Pattern] = &[
    // Secrets
    Pattern {
        name: "aws-access-key-id",
        category: Category::Secret,
        description: "AWS access key ID (long-term AKIA or temporary ASIA)",
        regex: r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b",
    },
    Pattern {
        name: "aws-secret-access-key",
        category: Category::Secret,
        description: "AWS secret access key assigned to an aws_secret_access_key-style name",
        regex: r#"(?i)aws_?secret_?access_?key["']?\s*[:=]\s*["']?[A-Za-z0-9/+=]{40}\b"#,
    },
    Pattern {
        name: "private-key",
        category: Category::Secret,
        description: "PEM/OpenSSH/PGP private key header",
        regex: r"-----BEGIN (?:(?:RSA|DSA|EC|OPENSSH|ENCRYPTED|PGP) )?PRIVATE KEY(?: BLOCK)?-----",
    },
    Pattern {
        name: "jwt",
        category: Category::Secret,
        description: "JSON Web Token (base64url JSON header and payload)",
        regex: r"\beyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+",
    },
    Pattern {
        name: "bearer-token",
        category: Category::Secret,
        description: "HTTP Bearer token of 20+ characters",
        regex: r"(?i)\bbearer\s+[A-Za-z0-9\-._~+/]{20,}=*",
    },
    Pattern {
        name: "github-token",
        category: Category::Secret,
        description: "GitHub personal access, OAuth, app, or refresh token",
        regex: r"\b(?:gh[pousr]_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{22,})\b",
    },
    Pattern {
        name: "slack-token",
        category: Category::Secret,
        description: "Slack bot, user, or app token",
        regex: r"\bxox[abposr]-[A-Za-z0-9-]{10,}",
    },
    Pattern {
        name: "secret-assignment",
        category: Category::Secret,
        description: "Value assigned to a password/secret/token/api_key-style name",
        regex: r#"(?i)\b(?:password|passwd|pwd|secret|token|api[_-]?key|access[_-]?key|private[_-]?key|client[_-]?secret|auth[_-]?token)["']?\s*[:=]\s*["']?[^\s"',;]{4,}"#,
    },
    Pattern {
        name: "url-credentials",
        category: Category::Secret,
        description: "URL with an embedded user:password",
        regex: r"\b[A-Za-z][A-Za-z0-9+.-]*://[^\s:/@]+:[^\s/@]+@",
    },
    // Personal data
    Pattern {
        name: "email",
        category: Category::Pii,
        description: "Email address",
        regex: r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b",
    },
    Pattern {
        name: "us-ssn",
        category: Category::Pii,
        description: "U.S. Social Security number (NNN-NN-NNNN)",
        regex: r"\b\d{3}-\d{2}-\d{4}\b",
    },
    Pattern {
        name: "us-phone",
        category: Category::Pii,
        description: "U.S. phone number (NNN-NNN-NNNN or (NNN) NNN-NNNN)",
        regex: r"(?:\b\d{3}-\d{3}-\d{4}\b|\(\d{3}\) \d{3}-\d{4}\b)",
    },
    Pattern {
        name: "credit-card",
        category: Category::Pii,
        description: "Visa, Mastercard, Amex, Discover, Diners, or JCB card number",
        regex: r"\b(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9]{2})[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35[0-9]{3})[0-9]{11})\b",
    },
    Pattern {
        name: "ipv4",
        category: Category::Pii,
        description: "IPv4 address",
        regex: r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
    },
    Pattern {
        name: "ipv6",
        category: Category::Pii,
        description: "Full (uncompressed) IPv6 address",
        regex: r"\b(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}\b",
    },
];

/// Compiles the patterns at `indices` (into `PATTERNS`).
pub fn compile(indices: &[usize]) -> Vec<Regex> {
    indices
        .iter()
        .map(|&i| Regex::new(PATTERNS[i].regex).expect("built-in pattern must compile"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regex(name: &str) -> Regex {
        let i = PATTERNS
            .iter()
            .position(|p| p.name == name)
            .unwrap_or_else(|| panic!("no pattern named {name}"));
        compile(&[i]).remove(0)
    }

    fn check(name: &str, hits: &[&str], misses: &[&str]) {
        let re = regex(name);
        for s in hits {
            assert!(re.is_match(s), "{name} should match {s:?}");
        }
        for s in misses {
            assert!(!re.is_match(s), "{name} should not match {s:?}");
        }
    }

    #[test]
    fn all_patterns_compile_and_names_are_unique() {
        let all: Vec<usize> = (0..PATTERNS.len()).collect();
        assert_eq!(compile(&all).len(), PATTERNS.len());
        let mut names: Vec<_> = PATTERNS.iter().map(|p| p.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), PATTERNS.len());
    }

    #[test]
    fn aws_access_key_id() {
        check(
            "aws-access-key-id",
            &["AKIAIOSFODNN7EXAMPLE", "id=ASIAIOSFODNN7EXAMPLE"],
            &[
                "AKIAIOSFODNN7EXAMPL",
                "AKIAIOSFODNN7EXAMPLEX",
                "AKIBIOSFODNN7EXAMPLE",
            ],
        );
    }

    #[test]
    fn aws_secret_access_key() {
        check(
            "aws-secret-access-key",
            &[
                "aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                r#""AWS_SECRET_ACCESS_KEY": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY""#,
            ],
            // A bare 40-character git SHA is not a finding.
            &["da39a3ee5e6b4b0d3255bfef95601890afd80709"],
        );
    }

    #[test]
    fn private_key() {
        check(
            "private-key",
            &[
                "-----BEGIN RSA PRIVATE KEY-----",
                "-----BEGIN OPENSSH PRIVATE KEY-----",
                "-----BEGIN PRIVATE KEY-----",
                "-----BEGIN ENCRYPTED PRIVATE KEY-----",
                "-----BEGIN PGP PRIVATE KEY BLOCK-----",
            ],
            &["-----BEGIN PUBLIC KEY-----", "-----BEGIN CERTIFICATE-----"],
        );
    }

    #[test]
    fn jwt() {
        check(
            "jwt",
            &[
                "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0In0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U",
            ],
            &["example.com", "file.txt", "a.b.c"],
        );
    }

    #[test]
    fn bearer_token() {
        check(
            "bearer-token",
            &["Authorization: Bearer abcdefghijklmnopqrstuvwxyz0123"],
            &["Bearer short", "bearer of bad news"],
        );
    }

    #[test]
    fn github_token() {
        check(
            "github-token",
            &[
                "ghp_abcdefghijklmnopqrstuvwxyz0123456789",
                "github_pat_11ABCDEFG0123456789_abcdefghijklmnop",
            ],
            &["ghp_tooshort", "ghx_abcdefghijklmnopqrstuvwxyz0123456789"],
        );
    }

    #[test]
    fn slack_token() {
        check(
            "slack-token",
            &["xoxb-1234567890-abcdefghij"],
            &["xoxz-1234567890-abcdefghij", "xoxb-short"],
        );
    }

    #[test]
    fn secret_assignment() {
        check(
            "secret-assignment",
            &[
                "password=hunter22",
                "PASSWORD = hunter22",
                r#"password: "p@ss!word""#,
                r#""api_key": "abcd1234""#,
                "export CLIENT_SECRET=s3cr3t!",
                "db.passwd: changeme",
            ],
            &[
                "password",
                "password=",
                "Enter your password below",
                "keyboard = qwerty",
                "monkey=banana",
            ],
        );
    }

    #[test]
    fn url_credentials() {
        check(
            "url-credentials",
            &[
                "postgres://admin:s3cret@db.example.com/app",
                "https://user:pass@example.com",
            ],
            &["https://example.com/path", "https://user@example.com"],
        );
    }

    #[test]
    fn email() {
        check(
            "email",
            &["contact me at jane.doe+tag@example.co.uk"],
            &["jane.doe@", "@example.com"],
        );
    }

    #[test]
    fn us_ssn() {
        check(
            "us-ssn",
            &["SSN: 123-45-6789"],
            &["123-456-7890", "1123-45-67890"],
        );
    }

    #[test]
    fn us_phone() {
        check(
            "us-phone",
            &["call 555-867-5309", "(555) 867-5309"],
            &["123-45-6789", "5558675309"],
        );
    }

    #[test]
    fn credit_card() {
        check(
            "credit-card",
            &[
                "4111111111111111",
                "card 5555555555554444 exp",
                "378282246310005",
            ],
            &[
                "41111111111111112",
                "1234567890123456",
                "The meeting is at 1030",
            ],
        );
    }

    #[test]
    fn ipv4() {
        check(
            "ipv4",
            &["host 192.168.1.10 up", "0.0.0.0"],
            &["256.1.1.1", "192.168.1"],
        );
    }

    #[test]
    fn ipv6() {
        check(
            "ipv6",
            &["2001:0db8:85a3:0000:0000:8a2e:0370:7334"],
            &["2001:db8::1", "12:34:56"],
        );
    }
}
