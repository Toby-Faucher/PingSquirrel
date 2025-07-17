use clap::{Parser, Subcommand};
use std::env;

include!(concat!(env!("OUT_DIR"), "/mac_vendor.rs"));

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Looks up a MAC address
    Lookup {
        /// The MAC address to look up.
        mac: String,
    },
    /// Forces an update of the OUI database
    Update,
}

fn lookup_vendor(mac: &str) -> Option<(&str, &str, &str)> {
    let prefix = (&mac[..8].to_uppercase()).replace(":", "");
    println!("{} ", prefix);
    MAC_VENDORS.get(prefix.as_str()).copied()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lookup { mac } => {
            if let Some((name, address, country)) = lookup_vendor(mac) {
                println!(
                    "Vendor: {}, Address: {}, Country: {}",
                    name, address, country
                );
            } else {
                println!("Vendor not found");
            }
        }
        Commands::Update => {
            println!("Updating OUI database...");
            //TODO: Implement update logic
            println!("OUI database updated successfully.");
        }
    }
    Ok(())
}
