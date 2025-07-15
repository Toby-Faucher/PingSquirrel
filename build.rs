use std::collections::HashMap;
use std::env;

use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct MacVendor {
    pub hex_prefix: String,
    pub base16_prefix: String,
    pub company_name: String,
    pub address_lines: Vec<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
}

impl MacVendor {
    pub fn new() -> Self {
        Self {
            hex_prefix: String::new(),
            base16_prefix: String::new(),
            company_name: String::new(),
            address_lines: Vec::new(),
            city: String::new(),
            state_province: String::new(),
            postal_code: String::new(),
            country: String::new(),
        }
    }
}

fn log_line(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("build_output.log")
        .expect("Could not open log file");

    writeln!(file, "{}", msg).expect("Could not write to log file");
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    // We get the OUI's from here
    let url = env::var("DATA_URL")
        .unwrap_or_else(|_| "https://standards-oui.ieee.org/oui/oui.txt".to_string());

    let content = fetch_content(&url)?;

    let data = parse_content(&content)?;

    generate_phm(&data)?;

    Ok(())
}

fn fetch_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching OUI's from: {}", url);

    let response = reqwest::blocking::get(url)?.text()?;

    Ok(response)
}

fn parse_content(content: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut data: HashMap<String, String> = HashMap::new();

    for (i, line) in content.lines().enumerate() {
        log_line(&format!("{}: {}", i, line));
    }
    Ok(data)
}

fn generate_phm(_data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
