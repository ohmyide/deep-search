use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
};
use std::ffi::OsStr;
use clap::{App, Arg};

use aiwriteclicheck:: {get_directory_list, search_in_file};

fn main() -> Result<(), Error> {
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new("rust-search")
        .version(version)
        .author("ohmyide - https://github.com/ohmyide/aiwriter-cli-check")
        .about("A Tiny, Static, Full-text Search with Rust")
        .arg(
            Arg::with_name("keyword")
                // .short('t')
                // .long("traverse")
                .value_name("KETWORD")
                .help("The keywords you want to search for")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("The file or folder you want to search")
                .required(false)
                .index(2)
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("case_ignore")
                .short('c')
                .multiple(false)
                .help("Case ignore"),
        )
        .arg(
            Arg::with_name("recursive")
                .short('r')
                .multiple(false)
                .help("Disable Recursion"),
        )
        .get_matches();

    
    let case_ignore = matches.is_present("case_ignore");
    let recursive = matches.is_present("recursive");
    let directory_path = matches.value_of("path");

    let root_path;
    if directory_path.is_none() {
        root_path = Path::new("./");
    } else {
        root_path = Path::new(directory_path.as_deref().unwrap_or("./"));
    }

    if let Some(keyword) = matches.value_of("keyword") {
        walk(&keyword, &root_path, recursive, case_ignore)?;
        Ok(())
    } else {
        eprintln!("Invalid keyword.");
        Result::Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid directory: keyword"),
        ))
    }

}

fn walk(keyword: &str, path: &Path, recursive: bool, case_ignore: bool) -> Result<(), Error> {
    let git_dir: &OsStr = OsStr::new(".git");
    let node_modules: &OsStr = OsStr::new("node_modules");
    if path.is_dir() {
        let dir_listing = get_directory_list(path);
        for entry in dir_listing {
            let temp_path = Path::new(&entry);
            let filename = temp_path.file_name();
            if temp_path.is_dir() {
                if recursive {
                    continue;
                }
                if filename != Some(git_dir) && filename != Some(node_modules) {
                    walk(&keyword, temp_path, recursive, case_ignore)?;
                }
            } else {
                search_in_file(&keyword, temp_path, case_ignore).ok();
            }
        }
    } else {
        search_in_file(&keyword, path, case_ignore).ok();
    }
    
    
    Ok(())
}