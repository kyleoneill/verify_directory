use std::env;
use std::path::Path;
use std::path::PathBuf;

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
    let directory_map = match generate_hashmap(config.first_dir.as_path(), &config.first_dir){
        Ok(res) => res,
        Err(_e) => return Err("IO Error creating map of original folder")
    };
    //For multiple copy comparison, change this to be a "for directory in config.comparison_dirs do compare_hashmap_to_directory"
    compare_hashmap_to_directory(directory_map, config.second_dir.as_path(), &config)
}
