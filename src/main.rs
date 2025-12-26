use std::io::{BufRead, BufReader, Result};
use std::{fs};

fn main() {
    let mut paths: Vec<String> = Vec::new();
    let mut ignored_paths: Vec<String> = Vec::new();
    get_ignored(&mut ignored_paths);
    get_paths("./", &mut paths);

    for path in paths {
        if should_ignore_path(&path, &ignored_paths) {
            continue;
        }

        let has_newline = match ends_with_newline(&path) {
            Ok(v) => v,
            Err(_) => {
                println!("error checking newline");
                continue
            }
        };

        if has_newline {
           println!("\x1b[32m[{path}] Ok\x1b[0m")
       } else {
           println!("\x1b[31m[{path}] Error - no new line on the end of file\x1b[0m")
       }
    }
}

fn should_ignore_path(path: &str, ignored_paths: &Vec<String>) -> bool {
    for ignored_path in ignored_paths {
        if path.contains(ignored_path) {
            return true
        }
    }
    return false
}

fn get_paths(path: &str, paths: &mut Vec<String>) {
    let paths_entries = fs::read_dir(path).unwrap();

    for entry in paths_entries {
        let path = entry.unwrap().path();
        let path_as_string = String::from(path.to_str().unwrap());
        if path.is_dir() {
            get_paths(&path_as_string, paths)
        } else {
            paths.push(path_as_string)
        }
    }
}

fn get_ignored(paths: &mut Vec<String>) {
    // *untitled.ignore* as the ignore file of untitled format checker
    let file = match fs::File::open("./untitled.ignore") {
        Ok(file) => file,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        paths.push(line.unwrap())
    }
}

fn ends_with_newline(path: &str) -> Result<bool> {
    let data = fs::read(path)?;
    Ok(matches!(data.last(), Some(b'\n')))
}
