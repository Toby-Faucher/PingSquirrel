include!(concat!(env!("OUT_DIR"), "/mac_vendor.rs"));

fn lookup_vendor(mac: &str) -> Option<(&str, &str, &str)> {
    let prefix = (&mac[..8].to_uppercase()).replace(":", "");
    println!("{} ", prefix);
    MAC_VENDORS.get(prefix.as_str()).copied()
}

fn main() {
    if let Some((name, address, country)) = lookup_vendor("6C:63:9C:B6:92:09") {
        println!(
            "Vendor: {}, Address: {}, Country: {}",
            name, address, country
        );
    } else {
        println!("Vendor not found");
    }
}
