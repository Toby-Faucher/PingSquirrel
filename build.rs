use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct MacVendor {
    pub base16_prefix: String,
    pub company_name: String,
    pub address_lines: Vec<String>,
    pub location: String,
    pub country: String,
}

impl MacVendor {
    pub fn new() -> Self {
        Self {
            base16_prefix: String::new(),
            company_name: String::new(),
            address_lines: Vec::new(),
            location: String::new(),
            country: String::new(),
        }
    }
}

fn extract_mac(line: &str) -> Option<&str> {
    if line.len() >= 6 {
        Some(&line[..6])
    } else {
        None
    }
}

fn extract_company_name(line: &str) -> Option<&str> {
    if let Some(index) = line.rfind("(base 16)") {
        Some((&line[(index + 9)..]).trim_start())
    } else {
        None
    }
}

fn extract_address(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if !trimmed.is_empty() {
        Some(trimmed)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-env-changed=DATA_URL");

    // We get the OUI's from here
    let url = env::var("DATA_URL")
        .unwrap_or_else(|_| "https://standards-oui.ieee.org/oui/oui.txt".to_string());

    let content = fetch_content(&url, false)?;

    let data = parse_content(&content)?;

    generate_phm(&data)?;

    Ok(())
}

fn fetch_content(url: &str, force: bool) -> Result<String, Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let cache_path = Path::new(&out_dir).join("oui_cache.txt");
    let cache_duration = Duration::from_secs(24 * 60 * 60); // 24 hours

    if !force && cache_path.exists() {
        if let Ok(metadata) = cache_path.metadata() {
            if let Ok(modified_time) = metadata.modified() {
                if SystemTime::now().duration_since(modified_time)? < cache_duration {
                    println!("Using cached OUI data from: {}", cache_path.display());
                    return Ok(std::fs::read_to_string(&cache_path)?);
                }
            }
        }
    }

    println!("Fetching OUI's from: {}", url);
    let content = reqwest::blocking::get(url)?.text()?;
    std::fs::write(&cache_path, &content)?;
    Ok(content)
}

fn parse_content(content: &str) -> Result<HashMap<String, MacVendor>, Box<dyn std::error::Error>> {
    let mut data: HashMap<String, MacVendor> = HashMap::new();
    let mut lines = content.lines().skip(4).peekable();

    while let Some(line) = lines.next() {
        if line.contains("(hex)") {
            continue;
        }
        if line.contains("(base 16)") {
            let mut mac_vendor = MacVendor::new();
            if let Some(mac) = extract_mac(line) {
                mac_vendor.base16_prefix = mac.to_string();
            } else {
                continue;
            }

            if let Some(company) = extract_company_name(line) {
                mac_vendor.company_name = company.to_string();
            } else {
            }

            let mut address_parts: Vec<String> = Vec::new();
            while let Some(next_line) = lines.peek() {
                if next_line.trim().is_empty()
                    || next_line.contains("(base 16)")
                    || next_line.contains("(hex)")
                {
                    break;
                }

                let address_line = lines.next().unwrap();
                if let Some(address) = extract_address(address_line) {
                    address_parts.push(address.to_string());
                }
            }

            if !address_parts.is_empty() {
                if let Some(country) = address_parts.pop() {
                    mac_vendor.country = country.to_string();
                }

                if let Some(location_line) = address_parts.pop() {
                    mac_vendor.location = location_line.to_string();
                }
            }
            data.insert(mac_vendor.base16_prefix.clone(), mac_vendor);
        }
    }
    Ok(data)
}

fn generate_phm(data: &HashMap<String, MacVendor>) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("mac_vendor.rs");

    let mut map = phf_codegen::Map::new();

    for (mac, vendor) in data {
        let value = format!(
            "({:?}, {:?}, {:?})",
            vendor.company_name, vendor.location, vendor.country
        );

        map.entry(mac, value);
    }

    let mut file = std::fs::File::create(dest_path)?;

    writeln!(
        &mut file,
        "static MAC_VENDORS: phf::Map<&'static str, (&'static str, &'static str, &'static str)> = {};",
        map.build()
    )?;
    println!(
        "Generated PHF MAC vendor lookup table with {} entries",
        data.len()
    );
    Ok(())
}
