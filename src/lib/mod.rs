use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Instant};

mod gen_hashmap;
use gen_hashmap::generate_hashmap;

mod compare_hashmap;
use compare_hashmap::compare_hashmap_to_directory;

extern crate data_encoding;
extern crate ring;

pub struct Config {
    pub first_dir: PathBuf,
    pub second_dir: PathBuf //For multiple copy comparison, change this to be a vec<PathBuf>
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let first_dir = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("Didn't get a folder name.")
        };
        let second_dir = match args.next() { //For multiple copy comparison, change this to push all remaining args to a vec<PathBuf>
            Some(arg) => PathBuf::from(arg),
            None => return Err("Didn't get a second folder name.")
        };
        if !Path::new(&first_dir).is_dir() && Path::new(&second_dir).is_dir() {
            return Err("One or both of the inputs is not a directory")
        }
        if first_dir == second_dir {
            return Err("Input folders are the same folder")
        }
        Ok(Config{first_dir, second_dir})
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let start = Instant::now();
    let (directory_map, directory_size) = match generate_hashmap(config.first_dir.as_path(), &config.first_dir){
        Ok(res) => res,
        Err(_e) => return Err("IO Error creating map of original folder")
    };
    //For multiple copy comparison, change this to be a "for directory in config.comparison_dirs do compare_hashmap_to_directory"
    let mismatcher = compare_hashmap_to_directory(directory_map, config.second_dir.as_path(), &config).unwrap();

    let (new_byte_size, byte_abbr) = format_bytes(directory_size as f64);
    let (time_elapsed, time_unit) = format_time(start);

    println!("Finished comparing {:.2} {} in {} {}\n", new_byte_size, byte_abbr, time_elapsed, time_unit);
    mismatcher.print_mismatches();
    Ok(())
}

fn format_bytes(bytes: f64) -> (f64, String) {
    if bytes > 2147483648.0 {
        (bytes / 2147483648.0, "GB".to_string())
    }
    else if bytes > 2097152.0 {
        (bytes / 2097152.0, "MB".to_string())
    }
    else if bytes > 2048.0 {
        (bytes / 2048.0, "KB".to_string())
    }
    else {
        (bytes, "B".to_string())
    }
}

fn format_time(start_time: Instant) ->  (u128, String) {
    let elapsed_time = start_time.elapsed();
    if elapsed_time.as_secs() > 0 {
        (elapsed_time.as_secs() as u128, "seconds".to_string())
    }
    else if elapsed_time.as_millis() > 0 {
        (elapsed_time.as_millis(), "milliseconds".to_string())
    }
    else if elapsed_time.as_micros() > 0 {
        (elapsed_time.as_micros(), "microseconds".to_string())
    }
    else {
        (elapsed_time.as_nanos(), "nanoseconds".to_string())
    }
}
