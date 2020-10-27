use std::io::Result;

#[allow(dead_code)]
struct AppConfig {
    watched_dirs: Vec<&'static str>,
    contribution_in_bytes: u64,
}

pub fn init() -> Result<()> {
    println!("Starting file monitoring");

    // Main daemon loop
    loop {}
}
