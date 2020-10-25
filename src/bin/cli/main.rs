use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Backup Fam")
        .version(VERSION)
        .subcommand(SubCommand::with_name("add-watch")
              .about("Adds a directory to be watched.")
              .arg(Arg::with_name("directory")
                  .takes_value(true)))
        .subcommand(SubCommand::with_name("set-contribution")
              .about("Sets the contribution size in bytes.")
              .arg(Arg::with_name("contribution")
                   .takes_value(true)))
        .get_matches();

    // TODO: add implementations when strategy for interacting with
    // background monitoring process has been determined. 
    match matches.subcommand_name() {
        Some("add-watch")        => {
            unimplemented!("add-watch command not implemented");
        },
        Some("set-contribution") => {
            unimplemented!("set-contribution command not implemented");
        },
        _ => {}
    }
}
