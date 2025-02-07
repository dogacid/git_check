use git2::Repository;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::{fs, process};
use std::path::Path;
use toml::Value;  // Add "toml" to Cargo.toml dependencies

fn main() {
    // Load config
    let config = fs::read_to_string(".git_check/config.toml")
        .expect("Failed to read .git_check/config.toml, create one in the repository, if no tthere");
    let config: Value = toml::from_str(&config).expect("Invalid TOML");

    let keywords: Vec<String> = config["keywords"].as_array()
        .expect("Missing 'keywords' array")
        .iter().map(|v| v.as_str().unwrap().to_string()).collect();

    let extensions: Vec<String> = config["extensions"].as_array()
        .expect("Missing 'extensions' array")
        .iter().map(|v| v.as_str().unwrap().to_string()).collect();

    // Open repository
    let repo = Repository::open(".").expect("Failed to open repository");

    let index = repo.index().expect("Failed to get repository index");
    let tracked_files: Vec<String> = index.iter()
        .filter_map(|entry| {
            let path = std::str::from_utf8(&entry.path).ok()?;
            Some(path.to_string())
        })
        .collect();

    // Search each file
    for file in tracked_files {
        if let Some(ext) = Path::new(&file).extension() {
            if extensions.contains(&ext.to_string_lossy().to_string()) {
                check_file(&file, &keywords);
            }
        }
    }
}

fn check_file(file: &str, keywords: &[String]) {
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => return,
    };

    let mut flagged = false;
    for (i, line) in content.lines().enumerate() {
        for keyword in keywords {
            if line.contains(keyword) {
                flagged = true;
                println!("\n🚨 Found '{}' in {}:{}", keyword, file, i + 1);
                println!("  {}", line.trim());
                if !confirm_action(file) {
                    println!("❌ Please fix before committing.");
                    process::exit(1);
                }
            }
        }
    }

    if flagged {
        println!("✅ No more issues found in {}", file);
    }
}

fn confirm_action(_file: &str) -> bool {
    print!("Should this file be checked in? (y/n): ");
    io::stdout().flush().unwrap();

    let tty = File::open("/dev/tty").expect("Failed to open /dev/tty");
    let mut reader = io::BufReader::new(tty);
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}