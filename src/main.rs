include!(concat!(env!("OUT_DIR"), "/mac_vendor.rs"));

fn lookup_vendor(mac: &str) -> Option<(&str, &str, &str)> {
    let prefix = (&mac[..8].to_uppercase()).replace(":", "");
    println!("{} ", prefix);
    MAC_VENDORS.get(prefix.as_str()).copied()
}

fn main() {
    if let Some((name, address, country)) = lookup_vendor("CC:BE:59:33:25:55") {
        println!(
            "Vendor: {}, Address: {}, Country: {}",
            name, address, country
        );
    } else {
        println!("Vendor not found");
    }
}
