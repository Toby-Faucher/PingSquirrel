
use std::env;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    // We get the OUI's from here
    let url = env::var("DATA_URL").unwrap_or_else(|_| {
        "https://standards-oui.ieee.org/oui/oui.txt".to_string()
    });

    let content = fetch_content(&url)?;

    let data = parse_content(&content)?;

    generate_phm(&data)?;

    Ok(())
}

fn fetch_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
   println!("Fetching OUI's from: {}", url); 

   let response = reqwest::blocking::get(url)?
       .text()?;

    Ok(response)
}


fn parse_content(_content: &str) -> Result<HashMap<String,String>, Box<dyn std::error::Error>> {
    Ok(HashMap::new())
}

fn generate_phm(_data: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
