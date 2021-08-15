use clap::{values_t, App, Arg, ArgMatches};
use postgres::{Client, NoTls};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::result::Result;
use walkdir::WalkDir;

const BASE_PATH: &str = "/home/tyagig/obunch/migrations/";

// (skip file list, force to drop and create)
#[derive(Debug)]
struct Ops(Vec<String>, bool);

impl Ops {
    fn new() -> Ops {
        let val: Vec<String> = Vec::default();
        Ops(val, false)
    }

    fn get_skip_values<'a>(&'a mut self) -> Vec<&'a str> {
        let mut files = Vec::new();
        for data in &self.0 {
            files.push(data.as_str().split(',').collect::<Vec<&str>>());
        }
        files.into_iter().flatten().collect::<Vec<&str>>()
    }

    fn new_from_matches(matches: ArgMatches) -> Ops {
        let mut ops = Self::new();
        let skip: Vec<String> = values_t!(matches.values_of("skip"), String).unwrap();
        ops.0 = skip;
        ops
    }
}

fn get_args_matches<'a>() -> ArgMatches<'a> {
    let args = &[Arg::with_name("skip")
        .short("s")
        .takes_value(true)
        .value_name("FILE")
        .multiple(true)];
    App::new("Migration program")
        .author("Gaurav Tyagi")
        .version("0.0.1")
        .about("A verry small and simple CLI to migrate sql files on DB")
        .args(args)
        .get_matches()
}

fn get_all_files(ops: &mut Ops) -> Result<Vec<PathBuf>, String> {
    let mut res = Vec::new();
    let skip_files = ops.get_skip_values();

    println!("File's pattern to skip: {:?}", skip_files);
    for data in WalkDir::new(BASE_PATH)
        .into_iter()
        .filter_entry(move |e| !skip_files.iter().any( |sf| e.path().to_str().unwrap().contains(sf)))
    {
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

fn get_db_conn() -> Client {
    let client = Client::connect(
        "host=localhost dbname=obunch user=gaurav password=test123",
        NoTls,
    )
    .unwrap();
    client
}

fn main() {
    let mut conn = get_db_conn();
    let matches = get_args_matches();
    let mut ops = Ops::new_from_matches(matches);

    let files = get_all_files(&mut ops);
    println!("{:?}", files);

    let mut execute_files = |files: Vec<PathBuf>| {
        for file in files {
            execute_file(file.as_os_str(), &mut conn);
        }
    };

    match files {
        Ok(files) => {
            execute_files(files);
        }
        Err(err) => println!("Error in getting file list: {:?}", err),
    }
}

