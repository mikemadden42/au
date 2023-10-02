# au

Search for gold in text files

This is my experiment of asking ChatGPT to help me write a Rust tool to hunt for gold in text files.  The inital steps are pretty basic, and I ask ChatGPT to help me build in more complex functionality.

NOTE: This tool should be used for authorized testing and/or educational purposes only.

In rust, how do I find all the text files in my current directory?

```rust
use std::fs;
use std::path::Path;

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            println!("Found text file: {path:?}");
                        }
                    }
                }
            }
        }
    }
}
```

In rust, how do I recursively find all the text files in my current directory?

```rust
use std::fs;
use std::path::Path;

fn find_text_files_recursive(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_text_files_recursive(&path);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            println!("Found text file: {path:?}");
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    find_text_files_recursive(&current_dir);
}
```

In rust, how do I recursively find all the text files in my current directory and apply a regular expression against the file's contents?

```rust
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::Path; // Add this line to import the regex crate

fn find_text_files_and_match_pattern_recursive(dir: &Path, pattern: &Regex) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_text_files_and_match_pattern_recursive(&path, pattern);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            match fs::File::open(&path) {
                                Ok(mut file) => {
                                    let mut content = String::new();
                                    if file.read_to_string(&mut content).is_ok()
                                        && pattern.is_match(&content)
                                    {
                                        // Regular expression match found
                                        println!("Found match in file: {path:?}");
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Failed to open file: {path:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Create a regular expression pattern (e.g., matching the word "example")
    let pattern = Regex::new(r"example").expect("Invalid regex pattern");

    find_text_files_and_match_pattern_recursive(&current_dir, &pattern);
}
```

In rust, how do I recursively find all the text files in my current directory and apply a series of regular expressions against the file's contents?

```rust
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::Path; // Add this line to import the regex crate

fn find_text_files_and_match_patterns_recursive(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_text_files_and_match_patterns_recursive(&path, patterns);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            match fs::File::open(&path) {
                                Ok(mut file) => {
                                    let mut content = String::new();
                                    if file.read_to_string(&mut content).is_ok() {
                                        // Check content against each pattern
                                        for pattern in patterns {
                                            if pattern.is_match(&content) {
                                                // Regular expression match found
                                                println!(
                                                    "Found match in file: {path:?} with pattern: {pattern:?}"
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Failed to open file: {path:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Create a list of regular expression patterns (e.g., matching "example" and "rust")
    let patterns = vec![
        Regex::new(r"example").expect("Invalid regex pattern"),
        Regex::new(r"rust").expect("Invalid regex pattern"),
    ];

    find_text_files_and_match_patterns_recursive(&current_dir, &patterns);
}

```

In rust, how do I recursively find all the text files in my current directory and apply a series of regular expressions for common passwords and tokens against the file's contents?

```rust
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::Path; // Add this line to import the regex crate

fn common_password_and_token_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"password=\w+").expect("Invalid regex pattern"), // Matches passwords
        Regex::new(r"token=\w+").expect("Invalid regex pattern"),    // Matches tokens
                                                                     // Add more patterns for common passwords and tokens as needed
    ]
}

fn find_common_passwords_and_tokens_recursive(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_common_passwords_and_tokens_recursive(&path, patterns);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            match fs::File::open(&path) {
                                Ok(mut file) => {
                                    let mut content = String::new();
                                    if file.read_to_string(&mut content).is_ok() {
                                        // Check content against each pattern
                                        for pattern in patterns {
                                            if pattern.is_match(&content) {
                                                // Match found
                                                println!(
                                                    "Found match in file: {:?} with pattern: {:?}",
                                                    path, pattern
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Failed to open file: {:?}", path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let patterns = common_password_and_token_patterns();

    find_common_passwords_and_tokens_recursive(&current_dir, &patterns);
}
```

In rust, what are some common regular expressions used to match passwords, tokens, and other secrets?

```rust
let password_pattern = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").expect("Invalid regex pattern");

let api_token_pattern = Regex::new(r"^[A-Za-z\d]{32}$").expect("Invalid regex pattern");

let email_pattern = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").expect("Invalid regex pattern");

let bearer_token_pattern = Regex::new(r"^Bearer [A-Za-z\d]+(\.[A-Za-z\d]+)*$").expect("Invalid regex pattern");

let sensitive_info_pattern = Regex::new(r"(password|token|secret|key)\s*=\s*[A-Za-z\d]+").expect("Invalid regex pattern");
```

In rust can you make these common patterns into a vector of regular expressions?

```rust
use regex::Regex;

fn main() {
    let patterns: Vec<Regex> = vec![
        Regex::new(r"^(?=.*[A-Za-z])[?=.*\d](A-Za-z\d){8,}$").expect("Invalid regex pattern"),
        Regex::new(r"^[A-Za-z\d]{32}$").expect("Invalid regex pattern"),
        Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").expect("Invalid regex pattern"),
        Regex::new(r"^Bearer [A-Za-z\d]+(\.[A-Za-z\d]+)*$").expect("Invalid regex pattern"),
        Regex::new(r"(password|token|secret|key)\s*=\s*[A-Za-z\d]+").expect("Invalid regex pattern"),
        // Add more patterns as needed
    ];

    // Example usage: Match against a string
    let input = "MyPassword123";
    for pattern in &patterns {
        if pattern.is_match(input) {
            println!("Matched pattern: {:?}", pattern);
        }
    }
}
```

I get this error.  What is wrong with it?

```bash
thread 'main' panicked at 'Invalid regex pattern: Syntax(
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
regex parse error:
    ^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$
     ^^^
error: look-around, including look-ahead and look-behind, is not supported
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
)', src/main.rs:8:63
```

```rust
use regex::Regex;

fn main() {
    let password_pattern = Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$").expect("Invalid regex pattern");

    // Rest of your code here...
}
```

This pattern should work as expected for password validation without relying on lookahead assertions, and it should not trigger any parse errors in Rust's regex crate.

```rust
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::Path; // Add this line to import the regex crate

fn common_password_and_token_patterns() -> Vec<Regex> {
    vec![
        // Basic Password Pattern
        Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$")
            .expect("Invalid regex pattern"),
        // API Tokens or API Keys
        Regex::new(r"^[A-Za-z\d]{32}$").expect("Invalid regex pattern"),
        // Email Addresses
        Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$")
            .expect("Invalid regex pattern"),
        // API Bearer Tokens
        Regex::new(r"^Bearer [A-Za-z\d]+(\.[A-Za-z\d]+)*$").expect("Invalid regex pattern"),
        // Sensitive Information
        Regex::new(r"(password|token|secret|key)\s*=\s*[A-Za-z\d]+")
            .expect("Invalid regex pattern"),
    ]
}

fn find_common_passwords_and_tokens_recursive(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_common_passwords_and_tokens_recursive(&path, patterns);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            match fs::File::open(&path) {
                                Ok(mut file) => {
                                    let mut content = String::new();
                                    if file.read_to_string(&mut content).is_ok() {
                                        // Check content against each pattern
                                        for pattern in patterns {
                                            if pattern.is_match(&content) {
                                                // Match found
                                                println!(
                                                    "Found match in file: {path:?} with pattern: {pattern:?}"
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Failed to open file: {path:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let patterns = common_password_and_token_patterns();

    find_common_passwords_and_tokens_recursive(&current_dir, &patterns);
}
```

For this program, I have a file called test.txt with the contents of Password123.  The program is not showing a match.  What is wrong?

To fix this issue, you can make the following adjustments to your code:

Trim leading and trailing whitespace from the file content.
Remove newline characters (e.g., \n) from the file content.

```rust
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn common_password_and_token_patterns() -> Vec<Regex> {
        vec![
        // Basic Password Pattern
        Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$")
            .expect("Invalid regex pattern"),

        // API Tokens or API Keys
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
        Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
            .expect("Invalid regex pattern"),

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
    ];
}

fn find_common_passwords_and_tokens_recursive(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_common_passwords_and_tokens_recursive(&path, patterns);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            if let Ok(file) = fs::File::open(&path) {
                                let reader = io::BufReader::new(file);

                                for line in reader.lines() {
                                    if let Ok(content) = line {
                                        // Trim leading and trailing whitespace, and remove newline characters
                                        let trimmed_content =
                                            content.trim_matches(|c| c == ' ' || c == '\n');

                                        // Check content against each pattern
                                        for pattern in patterns {
                                            if pattern.is_match(trimmed_content) {
                                                // Match found
                                                println!(
                                                    "Found match in file: {path} with pattern: {pattern}",
                                                    path = path.display(),
                                                    pattern = pattern
                                                );
                                            }
                                        }
                                    }
                                }
                            } else {
                                eprintln!("Failed to open file: {path:?}");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let patterns = common_password_and_token_patterns();

    find_common_passwords_and_tokens_recursive(&current_dir, &patterns);
}
```

Can you give an example word that would match each regular expression?

Basic Password Pattern (Minimum 8 characters with a mix of letters, digits, and symbols):

Word: P@ssw0rd
Explanation: This password includes a mix of uppercase letters, lowercase letters, digits, and symbols, and it is at least 8 characters long.
API Tokens or API Keys:

Word: 1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8
Explanation: This API token consists of 32 alphanumeric characters.
Email Addresses:

Word: <user@example.com>
Explanation: This is a valid email address following the standard format.
API Bearer Tokens:

Word: Bearer abc.def.ghi
Explanation: This is a valid API Bearer Token with parts separated by dots.
Sensitive Information:

Word: password=Secret123
Explanation: This string contains sensitive information in the form of "password=Secret123."
Strong Password Pattern (Minimum 8 characters with a mix of letters, digits, and symbols):

Word: P@ssw0rd!
Explanation: This password includes a mix of uppercase letters, lowercase letters, digits, and symbols, and it is at least 8 characters long.
UUID Pattern (Universally Unique Identifier):

Word: 3d8f7c0e-1c32-4c02-bf08-496e0cdd9252
Explanation: This is a valid UUID in the standard format.
Hexadecimal Color Code:

Word: #1a2b3c
Explanation: This is a valid hexadecimal color code.
JSON Web Token (JWT):

Word: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.6v8lCvWz3cRKvLbGzI40dP--6jMlfE8zy-kXkNNhFao
Explanation: This is a valid JWT with base64-encoded parts separated by dots.
Credit Card Number:

Word: 4111111111111111
Explanation: This is a valid credit card number in one of the recognized formats (Visa).
IPv4 Address:

Word: 192.168.0.1
Explanation: This is a valid IPv4 address in standard dotted-decimal notation.
These examples demonstrate strings that match each of the provided regular expressions.

How could I print out the line that matched?  It currently now printing out the exact line that matches.

```rust
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn common_password_and_token_patterns() -> Vec<Regex> {
    vec![
        // Basic Password Pattern
        Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$").expect("Invalid regex pattern"),

        // API Tokens or API Keys
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
    ]
}

fn find_and_print_matching_lines(dir: &Path, patterns: &[Regex]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively search subdirectories
                    find_and_print_matching_lines(&path, patterns);
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" || extension == "csv" {
                            // This is a text file
                            if let Ok(file) = fs::File::open(&path) {
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
                    }
                }
            }
        }
    }
}

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let patterns = common_password_and_token_patterns();

    find_and_print_matching_lines(&current_dir, &patterns);
}
```

For this rust program, can you generate a set of test words that will match all the regular expressions?

Generating a set of test words that match all the regular expressions can be challenging, especially for complex patterns. However, I can provide you with some examples that are likely to match each of the regular expressions in your program:

Basic Password Pattern:
MyPass123
StrongP@ss

API Tokens or API Keys (32 alphanumeric characters):
aabbccddeeff00112233445566778899
myapitoken1234567890abcdef12345678

Email Addresses:
<user@example.com>
<john.doe12345@gmail.com>

API Bearer Tokens:
Bearer abcdefghijklmnopqrstuvwxyz0123456789
Bearer jwt.token.here

Sensitive Information (with common keywords):
password=mysecretpassword
token=myapitoken123
secret=mysupersecret

Strong Password Pattern (Minimum 8 characters with a mix of letters, digits, and symbols):
P@ssw0rd
S3cureP@ss!

UUID Pattern (Universally Unique Identifier):
3a76bb1d-06d1-46eb-9a3e-e06a9467259f
0f5b43e2-d2d8-42ed-a272-6e25d3a7b8c0

Hexadecimal Color Code:

`#FF5733`

`#00AABB`

JSON Web Token (JWT):
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c

Credit Card Number:
4111111111111111
5555555555554444

IPv4 Address:
192.168.0.1
10.0.0.1

Please note that these examples are meant for testing purposes and may not cover all edge cases. Depending on your specific requirements, you may need to generate additional test cases to thoroughly test your regular expressions.

Any further suggestions for this program?

Your program looks well-structured and functional for its intended purpose. However, here are a few additional suggestions and considerations:

1. **Error Handling**: Your program currently uses `.expect("Invalid regex pattern")` for each `Regex` creation. While this will panic if there's an issue with the regex pattern, it might be more helpful to handle these errors gracefully. You can use the `Result` type to propagate errors up the call stack and provide more meaningful error messages.

   ```rust
   fn common_password_and_token_patterns() -> Result<Vec<Regex>, regex::Error> {
       let patterns = vec![
           // ... (other patterns)

           // Example: Basic Password Pattern
           Regex::new(r"^[A-Za-z\d]*[A-Za-z][A-Za-z\d]*\d[A-Za-z\d]*$")?,
       ];

       Ok(patterns)
   }
   ```

2. **Parallel Processing**: If you have a large number of files to search through, you might consider parallelizing the process to improve performance. The Rust standard library provides facilities for working with threads (`std::thread`) and asynchronous programming (`async`/`await`) using libraries like `tokio` or `async-std`.

3. **Configuration Options**: Consider adding command-line arguments or a configuration file to allow users to specify the directory to search in and other options. This would make the program more flexible.

4. **Logging**: Instead of using `eprintln!` for error messages, you could use a logging framework like `log` and configure different log levels. This makes it easier to manage logs in more complex applications.

5. **Testing**: Writing unit tests and integration tests for your code can help ensure its correctness and maintainability. Rust has good support for testing built into the standard library.

6. **Comments and Documentation**: While your code is fairly self-explanatory, adding comments to complex parts or generating documentation with `rustdoc` can make it more understandable for others (and future you).

7. **Cargo.toml Dependencies**: Ensure that you have specified your dependencies correctly in the `Cargo.toml` file, especially for the `regex` crate.

8. **Version Control**: If you're not already using version control (e.g., Git), consider doing so. It's a good practice for tracking changes to your codebase.

9. **Code Organization**: As your program grows, you might consider organizing your code into separate modules for better maintainability.

10. **Performance**: If performance is a concern, you can look into optimizing the regex patterns or using more efficient data structures or algorithms for the searching process.

These suggestions are aimed at improving code quality, maintainability, and extensibility. Depending on your specific use case and future plans for the program, you can prioritize and apply the ones that make the most sense for your needs.

How could I add parallel processing to this code?

```rust
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
```
