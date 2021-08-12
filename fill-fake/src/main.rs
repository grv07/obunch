use postgres::{Client, NoTls};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::result::Result;
use walkdir::{WalkDir, DirEntry};

use clap::{App, ArgMatches, Arg, Values};

const BASE_PATH: &str = "/home/tyagig/obunch/migrations/";

// (skip file list, force to drop and create)
#[derive(Debug)]
struct Ops<'a>(Values<'a>, bool);

fn parse_args<'a>() -> ArgMatches<'a> {
    let args = &[
        Arg::with_name("skip").short("s").takes_value(true).value_name("FILE").multiple(true),
        Arg::with_name("force").short("f")
    ];
    App::new("Migration program")
        .author("Gaurav Tyagi")
        .version("0.0.1")
        .about("A verry small and simple CLI to migrate sql files on DB")
        .args(args).get_matches()
}

fn get_all_files(ops: &mut Ops) -> Result<Vec<PathBuf>, String> {
    let mut res = Vec::new();
    let mut exclude_files = |de: &DirEntry| { ops.0.map(|file_name| file_name.split(",")).any(|f| f == de.file_name())}; 
    for data in WalkDir::new(BASE_PATH).into_iter().filter_entry(|e| !exclude_files(e)) {
        if let Ok(entry) = data {
            println!(" ===== {:?}", entry.file_name());
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

    let mut ops = Ops(Values::default(), false);
    let matches = parse_args();
    if matches.is_present("skip") {
        if let Some(s) = matches.values_of("skip") {
            ops.0 = s;
        }
    }

    if matches.is_present("force") {
        ops.1 = true;
    }
    println!(">>>>>>> {:?}", ops);

    let files = get_all_files(&mut ops);
    println!("{:?}", files);
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

