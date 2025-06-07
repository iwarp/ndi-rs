fn main() {
    ndi::initialize().unwrap();
    let builder = ndi::FindBuilder::new();
    let find = builder.show_local_sources(true).build().unwrap();
    println!("Looking for sources");
    let sources = find.current_sources(5000).unwrap();

    if sources.is_empty() {
        panic!("No sources found");
    }

    println!("Discovered Sources:");
    for src in &sources {
        let name = src.get_name();
        let url = src.get_url_address().unwrap_or_else(|| "<unknown>".to_string());
        let ip = src.get_ip_address();
        match ip {
            Some(ip) => println!("  {} - {} ({})", name, url, ip),
            None => println!("  {} - {}", name, url),
        }
    }

    println!("Done");

    unsafe {
        ndi::cleanup();
    }
}
