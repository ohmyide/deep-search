use std::{
    fs::{self},
    io::{Error},
    path::Path,
};
// use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use content_inspector::{ContentType};

const MAX_PEEK_SIZE: usize = 64;

pub fn search_in_file(keyword: &str, path: &Path, case_ignore: bool) -> Result<(), Error> {
    let extensioner = path.extension();
    if !extensioner.is_none() && path.exists(){
        let file = File::open(&path).expect("Unable to read file");
        let mut buffer: Vec<u8> = vec![];

        file.take(MAX_PEEK_SIZE as u64).read_to_end(&mut buffer)?;
        let content_type = content_inspector::inspect(&buffer);
        if content_type == ContentType::UTF_8 {
            let mut file = File::open(path)?;
            let mut buf = vec![];

            file.read_to_end (&mut buf)?;
            let contents = String::from_utf8_lossy (&buf);
            let results;
            if case_ignore {
                results = search_case_insensitive(&keyword, &contents);
            } else {
                results = search(&keyword, &contents);
            }
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let mut index = 0;
            if results.len() > 0 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(215,102,239))))?;
                println!("{}:",path.display());
            }
            // 输出搜索结果
            for line in results {
                index = index + 1;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                print!("{index}: ");
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                println!("{}",line);
                println!("");
            }
        }
    }
    Ok(())
}

pub fn get_directory_list(directory_path: &Path) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap()
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect()
}


pub fn search<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line|
            line.contains(keyword)
        )
        .collect()
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// // #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_search() {
//         assert_eq!(2 + 2, 4);
//     }
// }
