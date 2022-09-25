fn main() {
    ndi::initialize().unwrap();
    let builder = ndi::FindBuilder::new();
    let find = builder.show_local_sources(true).build().unwrap();
    println!("Looking for sources");
    let sources = find.current_sources(5000).unwrap();

    if sources.len() == 0 {
        panic!("No sources found");
    }

    println!("Discovered Sources {:?}", sources);

    println!("Done");

    unsafe {
        ndi::cleanup();
    }
}
