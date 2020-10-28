use clap::{App, Arg};
mod app;

use std::io::Result;
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let matches = App::new("Backup Fam -- Monitoring")
        .version(VERSION)
        .arg(
            Arg::with_name("directory")
                .short("d")
                .multiple(true)
                .takes_value(true)
        )
        .get_matches();

    let watched_dirs: Vec<PathBuf> = matches
        .values_of("directory")
        .unwrap()
        .map(|dir_arg| PathBuf::from(dir_arg))
        .collect();
    
    let config = app::Config {
        watched_dirs,
        // TODO: add contribution arg handling and byte unit ('gb', 'mb', ...) parsing.
        contribution_in_bytes: 10
    };
    
    app::init(&config)?;
    Ok(())
}
