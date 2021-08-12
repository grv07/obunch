use postgres::{Client, NoTls};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::result::Result;
use walkdir::WalkDir;

const BASE_PATH: &str = "/home/tyagig/obunch/migrations/";

fn get_all_files() -> Result<Vec<PathBuf>, String> {
    let mut res = Vec::new();
    for data in WalkDir::new(BASE_PATH).into_iter() {
        if let Ok(entry) = data {
            if entry.path().is_file() {
                let entry = entry.into_path();
                res.push(entry);
            }
        }
    }
    Ok(res)
}

fn execute_file(file_name: &OsStr, client: &mut Client) {
    let query = std::fs::read_to_string(file_name).unwrap();
    let result = client.batch_execute(&query);
    if result.is_err() {
        println!("Error in file {:?} {:?}", &file_name, result.err());
    }
}

fn main() {
    let mut client = Client::connect(
        "host=localhost dbname=obunch user=gaurav password=test123",
        NoTls,
    )
    .unwrap();
    let files = get_all_files();
    let mut execute_files = |files: Vec<PathBuf>| {
        for file in files {
            execute_file(file.as_os_str(), &mut client);
        }
    };
    match files {
        Ok(files) => {
            execute_files(files);
        }
        Err(err) => println!("Error in getting file list: {:?}", err),
    }
}
