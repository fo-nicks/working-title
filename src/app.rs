use std::io::Result;
use std::path::PathBuf;

pub struct Config {
    pub watched_dirs: Vec<PathBuf>,
    pub contribution_in_bytes: u64,
}

impl Config {
    pub fn print(&self) {
        println!("Configured watched directories");
        self.watched_dirs.iter().for_each(|dir_path| {
            println!("    - {}", dir_path.to_str().unwrap());
        });
    }
}

pub fn init(config: &Config) -> Result<()> {
    println!("Starting file monitoring");
    config.print();

    // Main daemon loop
    loop {}
}
