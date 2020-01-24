extern crate config;
extern crate serde;

use crypto::sha2::Sha512;
use crypto::digest::Digest;
use walkdir::WalkDir;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::{BufReader, Read};
use std::env;

mod datastore;
mod settings;

use datastore::*;
use settings::Settings;

#[macro_use]
extern crate serde_derive;

fn get_file_info(path: &str) -> Option<FileInfo> {
    let srcdir = PathBuf::from(&path);
    let full_path = fs::canonicalize(&srcdir).expect("File could not be processed");
    let file = File::open(&full_path).expect("can't upen file");
    let meta = file.metadata().unwrap();
    if meta.is_dir() {
        return None;
    }
    let file_length = meta.len();
    let mut reader = BufReader::new(file);

    let mut hasher = Sha512::new();
    
    let mut buffer = [0; 4096];

    loop {
        let count = reader.read(&mut buffer).expect("what kind of error can happen on reading buffer?");
        if count == 0 {
            break;
        }
        hasher.input(&buffer[0..count]);
    }    
    
    let digest = hasher.result_str();    

    Some(FileInfo {
        full_path : full_path.to_str().expect("Path could not be translated").to_string(),
        size : file_length,
        hash : digest
    })
}
fn process_path( settings: Settings) {
    
    'filewalker: for entry in WalkDir::new(settings.working_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_str().unwrap();
        let s_path = String::from(path);
        for s in  &settings.ignore_paths {
            if s_path.contains(s) {
                continue 'filewalker;
            }
        }
        if !entry.file_type().is_dir() {
            match get_file_info(path) {
                Some(info) => {
                    let mut file_already_added = false;
                    let data_for_path = get_entry_for_path(&info.full_path).expect("I assume None but not error!");
                    match data_for_path {
                        Some(d) => {
                            file_already_added = true;
                            if d.hash != info.hash {
                                println!("HASH changed for file : {} ! ", info.full_path);
                            }
                        }
                        None => ()
                    }
                    let possible_duplicates = get_entries_by_hash(&info.hash).expect("get_entries failed");
                    for dup_info in possible_duplicates.iter() {
                        if info.full_path != dup_info.full_path {
                            if info.hash == dup_info.hash && info.size == dup_info.size {
                                println!("Hashes are the same for files : {} and {} ! ", info.full_path, dup_info.full_path);
                            }     
                        }
                    }                   
                    if !file_already_added {
                        add_entry(&info).expect("Unable to add entry to db");
                    }

                    println!("file: {}", info.full_path );
                }
                None => println!("File at path {} was not processed", path)
            }            
            
        }
        
    }
}
fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new();
    create_tables().expect("I couldn't create tables!");        
    if settings.is_ok() {
        process_path(settings.unwrap());
    } else
    if let Some(arg) = env::args().nth(1) {
        process_path(Settings{ 
                  ignore_paths : vec![], 
                  working_dir : String::from(arg)
                });
        
    } else {
        println!("USAGE: duplicates PATH_TO_CHECK")

    }    
    
    Ok(())
}
