use std::{
    env,
    fs::{self},
    io::{Error, ErrorKind},
    path::Path,
};
// use std::error::Error;

use indextree::{Arena, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PathNode {
    name: String,
    relative_path: String,
    absolute_path: String,
    node_type: NodeType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum NodeType {
    File,
    Directory,
}

fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        // return Error("not enough arguments");
        print!("not enough arguments");
    }

    let query = args[1].clone();
    let directory_path = args[2].clone();

    let root_path = Path::new(&directory_path);

    if root_path.is_dir() {
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(root_path);

        let arena = &mut Arena::new();

        let root_node = arena.new_node(PathNode {
            name: directory_path.to_string(),
            node_type: NodeType::Directory,
            relative_path: directory_path.to_string(),
            absolute_path: absolute_path.display().to_string(),
        });

        let recursive = false;
        traverse(&query, &directory_path, arena, root_node, recursive)?;
        Ok(())
    } else {
        eprintln!("Invalid directory.");
        Result::Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid directory: {}", directory_path),
        ))
    }
}

fn traverse(
    query: &str,
    path: &str,
    arena: &mut Arena<PathNode>,
    parent: NodeId,
    recursive: bool,
) -> Result<(), Error> {
    // println!("*****************absolute_path{}")
    let dir_listing = get_directory_listing(path);
    for entry in dir_listing {
        let temp_path = Path::new(entry.as_str());
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(temp_path);

        if temp_path.is_dir() {
            let dir_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                node_type: NodeType::Directory,
            });

            parent.append(dir_object, arena);

            if recursive {
                traverse(&query, entry.as_str(), arena, dir_object, recursive)?;
            }
        } else {
            let file_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                node_type: NodeType::File,
            });

            let relative = absolute_path.display().to_string();
            // let relative = String::from(entry.as_str());
            // let mut absolute_path = std::env::current_dir()?;
            // absolute_path.push(&relative);
            let path = Path::new(&relative);

            let contents = fs::read_to_string(path)?;
            // let query = "卡片";
            let results = search(&query, &contents);

            // 输出搜索结果
            for line in results {
                println!("===== search result begin ====");
                println!("{}", line);
                println!("===== search result end ======");
                println!("");
            }

            parent.append(file_object, arena);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn get_directory_listing(directory_path: &str) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap()
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect()
}