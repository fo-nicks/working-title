use clap::{App, Arg, SubCommand};
use std::path::Path;

mod watched_fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Backup Fam")
        .version(VERSION)
        .subcommand(
            SubCommand::with_name("add-watch")
                .about("Adds a directory to be watched.")
                .arg(Arg::with_name("directory").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("set-contribution")
                .about("Sets the contribution size in bytes.")
                .arg(Arg::with_name("contribution").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("try-fuse")
                .about("Try out fuse.")
                .arg(
                    Arg::with_name("backing directory")
                        .short("-b")
                        .long("--backing-directory")
                        .required(true)
                        .takes_value(true)
                        .help("The backing directory for the FUSE mount."),
                )
                .arg(
                    Arg::with_name("mount directory")
                        .short("-m")
                        .long("--mount-directory")
                        .required(true)
                        .takes_value(true)
                        .help("The mount directory for the FUSE mount."),
                ),
        )
        .get_matches();

    // TODO: add implementations when strategy for interacting with
    // background monitoring process has been determined.
    match matches.subcommand() {
        ("add-watch", _) => {
            unimplemented!("add-watch command not implemented");
        }
        ("set-contribution", _) => {
            unimplemented!("set-contribution command not implemented");
        }
        ("try-fuse", Some(sub_matches)) => {
            let mount_dir = sub_matches.value_of("mount directory").unwrap();
            let backing_dir = sub_matches.value_of("backing directory").unwrap();
            println!(
                "Trying fuse, will mount at {}, backing to {}",
                mount_dir, backing_dir
            );
            let mount_dir = Path::new(mount_dir);
            let backing_dir = Path::new(backing_dir);
            match (check_directory(&mount_dir), check_directory(&backing_dir)) {
                (Ok(_), Ok(_)) => {
                    watched_fs::watch_filesystem(mount_dir, backing_dir);
                }
                (Err(mount_dir_message), _) => {
                    eprintln!("Mount directory is bad: {}", mount_dir_message);
                }
                (_, Err(backing_dir_message)) => {
                    eprintln!("Mount directory is bad: {}", backing_dir_message);
                }
            }
        }
        _ => {}
    }
}

/// Checks a potential directory for existence and directory-ness.
fn check_directory(directory: &Path) -> Result<(), String> {
    if !directory.exists() {
        Err(directory.to_string_lossy().into_owned() + " does not exist!")
    } else if !directory.is_dir() {
        Err(directory.to_string_lossy().into_owned() + " is not a directory!")
    } else {
        Ok(())
    }
}
