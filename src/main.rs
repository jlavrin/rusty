use std::io::{BufRead, BufReader, Read};
use std::{fs, io};
use std::path::Component::ParentDir;

fn main() {
    let mut paths: Vec<String> = Vec::new();
    let mut ignored_paths: Vec<String> = Vec::new();
    get_ignored(&mut ignored_paths);
    get_paths("./", &mut paths);

    for path in paths {
        let last = match get_last_line(&path, &ignored_paths) {
            Ok(None) => continue,
            Ok(res) => res.unwrap_or_default(),
            Err(_) => {
                println!("error getting last line");
                continue;
            }
        };

       if last == "" {
           println!("\x1b[32m[{path}] Ok\x1b[0m")
       } else {
           println!("\x1b[31m[{path}] Error - no new line on the end of file\x1b[0m")
       }
    }
}

fn get_last_line(path: &str, ignored_paths: &Vec<String>) -> io::Result<Option<String>> {
    for ignored_path in ignored_paths {
        if (path.contains(ignored_path)) {
            return Ok(None);
        }
    }

    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);

    let last = reader
        .lines()
        .last()
        .transpose()?;

    Ok(last)
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
