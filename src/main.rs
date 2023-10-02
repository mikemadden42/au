use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;

fn common_password_and_token_patterns() -> Vec<Regex> {
    vec![
        // Basic Password Pattern
        Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$")
            .expect("Invalid regex pattern"),

        // API Tokens or API Keys (32 alphanumeric characters)
        Regex::new(r"^[A-Za-z\d]{32}$").expect("Invalid regex pattern"),

        // Email Addresses
        Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$")
            .expect("Invalid regex pattern"),

        // API Bearer Tokens
        Regex::new(r"^Bearer [A-Za-z\d]+(\.[A-Za-z\d]+)*$").expect("Invalid regex pattern"),

        // Sensitive Information
        Regex::new(r"(password|token|secret|key)\s*=\s*[A-Za-z\d]+")
            .expect("Invalid regex pattern"),

        // Strong Password Pattern (Minimum 8 characters with a mix of letters, digits, and symbols)
        // Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").expect("Invalid regex pattern"),
        Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").expect("Invalid regex pattern"),

        // UUID Pattern (Universally Unique Identifier)
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .expect("Invalid regex pattern"),

        // Hexadecimal Color Code
        Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").expect("Invalid regex pattern"),

        // JSON Web Token (JWT)
        Regex::new(r"^[A-Za-z0-9-_]+(\.[A-Za-z0-9-_]+)*\.[A-Za-z0-9-_]+\s*$")
            .expect("Invalid regex pattern"),

        // Credit Card Number
        Regex::new(r"^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\d{3})\d{11})$")
            .expect("Invalid regex pattern"),

        // IPv4 Address
        Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$")
            .expect("Invalid regex pattern"),

        // SSH Private Key
        Regex::new(r"-----BEGIN (RSA|DSA|EC|OPENSSH) PRIVATE KEY-----").expect("Invalid regex pattern"),

        // AWS Access Key ID
        Regex::new(r"AKI[0-9A-Z]{16}").expect("Invalid regex pattern"),

        // AWS Secret Access Key
        Regex::new(r"[0-9a-zA-Z/+=]{40}").expect("Invalid regex pattern"),

        // Phone Numbers (U.S.)
        Regex::new(r"(\d{3}-\d{3}-\d{4}|\(\d{3}\) \d{3}-\d{4})").expect("Invalid regex pattern"),

        // Credit Card Expiration Date (MM/YY)
        Regex::new(r"^(0[1-9]|1[0-2])/(2[2-9]|[3-9][0-9])$").expect("Invalid regex pattern"),

        // URLs (HTTP/HTTPS)
        Regex::new(r"https?://[^\s/$.?#].[^\s]*").expect("Invalid regex pattern"),

        // Social Security Numbers (SSN)
        Regex::new(r"\d{3}-\d{2}-\d{4}").expect("Invalid regex pattern"),

        // IPv6 Addresses
        Regex::new(r"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}").expect("Invalid regex pattern"),

        // Credit Card CVV (Card Verification Value)
        Regex::new(r"\d{3,4}").expect("Invalid regex pattern"),

        // Date (YYYY-MM-DD)
        Regex::new(r"\d{4}-\d{2}-\d{2}").expect("Invalid regex pattern"),
        
        // Bank Account Numbers (International)
        Regex::new(r"^[0-9]{8,20}$").expect("Invalid regex pattern"),
        
        // Passport Numbers (e.g., U.S. passport number)
        Regex::new(r"^[0-9A-Za-z]{7,9}$").expect("Invalid regex pattern"),
        
        // Healthcare Identification Numbers (e.g., SSN in the U.S.)
        Regex::new(r"\d{3}-\d{2}-\d{4}").expect("Invalid regex pattern"),
        
        // Vehicle Identification Numbers (VIN)
        Regex::new(r"[A-HJ-NPR-Z0-9]{17}").expect("Invalid regex pattern"),
        
        // Phone Numbers (International)
        Regex::new(r"(\d{3}-\d{3}-\d{4}|\(\d{3}\) \d{3}-\d{4})").expect("Invalid regex pattern"),
        
        // URLs (HTTP/HTTPS)
        Regex::new(r"https?://[^\s/$.?#].[^\s]*").expect("Invalid regex pattern"),
        
        // Add your additional patterns here
    ]
}

fn process_file(path: &Path, patterns: &[Regex]) {
    if let Ok(file) = fs::File::open(path) {
        let reader = io::BufReader::new(file);

        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(content) = line {
                // Check content against each pattern
                for pattern in patterns {
                    if pattern.is_match(&content) {
                        // Match found
                        println!(
                            "Found match in file: {path:?}, line {line_number}: {content}",
                            path = path.display(),
                            line_number = line_number + 1,
                            content = content
                        );
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open file: {path:?}");
    }
}

fn find_and_print_matching_lines(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        let entries: Vec<_> = entries.filter_map(Result::ok).collect(); // Collect into a Vec

        entries.into_par_iter().for_each(|entry| {
            let path = entry.path();

            if path.is_dir() {
                // Recursively search subdirectories
                find_and_print_matching_lines(&path, patterns);
            } else if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "txt" || extension == "csv" {
                        // This is a text file
                        process_file(&path, patterns);
                    }
                }
            }
        });
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let patterns = common_password_and_token_patterns();

    find_and_print_matching_lines(&current_dir, &patterns);
}
