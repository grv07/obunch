use std::ffi::OsString;
use walkdir::{Error, WalkDir};

const BASE_PATH: &str = "/home/tyagig/obunch/migrations/";

fn get_all_files() -> Result<Vec<OsString>, Error> {
    let mut res = Vec::new();
    for data in WalkDir::new(BASE_PATH).into_iter() {
        if let Ok(entry) = data {
            if entry.path().is_file() {
                let entry = entry.file_name().to_owned();
                res.push(entry);
            }
        }
    }
    Ok(res)
}

fn main() {
    let files = get_all_files();
    println!("{:?}", files);
}
