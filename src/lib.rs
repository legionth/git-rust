use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::fs::metadata;
use chrono::format::Item::Error;

pub struct Repository {
    name: String,
    history: Vec<Commit>
}

pub struct Commit {
    id: String,
    message: String
}

impl Repository {
    pub fn new(name: String) -> Repository
    {
        Repository {
            name,
            history: vec![]
        }
    }
}
impl Commit {
    pub fn new(message: String) -> Commit
    {
        let id = String::from("1d");

        Commit {
            id,
            message
        }
    }
}

pub fn commit(message: String) -> io::Result<()> {
    let file = File::open(".gitrust/index").expect("Unable to open");
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

pub fn init() -> io::Result<()> {
    std::fs::create_dir(".gitrust")
}

pub fn add(path: String) -> io::Result<()> {
    let meta_data = metadata(&*path).unwrap();

    if meta_data.is_file() {
        let current_time = chrono::offset::Local::now();

        let mut content: String = String::from(current_time.to_string());
        content.push_str(&*" ".to_string());
        content.push_str(&*path);
        content.push_str(&*"\n".to_string());

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(".gitrust/index")
            .unwrap();

        return file.write_all(content.as_ref());
    }
    else if meta_data.is_dir() {
        let paths = fs::read_dir(path).unwrap();
        for sub_path in paths {
            let entry = sub_path.unwrap();
            let entry_path = entry.path();
            let path_as_string = entry_path.to_str().unwrap();
            add(path_as_string.to_string()).unwrap();
        }
    }

    return Ok(());
}

fn in_repository(repository: Repository) -> bool {
    std::path::Path::new(".gitrust").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let history = Vec::new();
        let mut repository = Repository {
            name: "new-project".to_string(),
            history
        };

        let old_length = repository.history.len();

        commit(&mut repository, "new commit".to_string());
        let new_length = repository.history.len();

        assert_eq!(old_length, 0);
        assert_eq!(new_length, 1);
    }
}