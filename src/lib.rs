use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::fs::metadata;
use std::ops::Add;
use sha2::{Sha256, Digest};


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
        let mut hasher = Sha256::new();

        let file_content = fs::read_to_string(&*path)
            .expect("Should have been able to read the file");

        hasher.update(file_content);
        let hash_result = format!("{:X}", hasher.finalize());;

        content.push_str(&*" ".to_string());
        content.push_str(&*path);
        content.push_str(" ");
        content.push_str(&*hash_result);
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

#[cfg(test)]
mod tests {
    use super::*;
}