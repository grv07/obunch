use std::fs::*;
use std::io::Result;
use std::path::Path;

const BASE_PATH: &str = "/home/tyagig/obunch/migrations";

// instead of creating the wheel again use WalkDir.
fn get_all_files() -> Result<Vec<File>> {
    let migrations: Vec<_> = read_dir(Path::new(BASE_PATH))?
        .map(|res| {
            res.map(|res| {
               if res.file_type().unwrap().is_dir() {
                    "true"} else { "false"}
            })})
        .collect();
    println!("{:?}", migrations);
    todo!()
}
fn main() {
    println!("Hello, world!");
    get_all_files();
}
