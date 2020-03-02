use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use super::Config;

use super::gen_hashmap::generate_hashmap;

extern crate data_encoding;
extern crate ring;

use ring::digest::Digest;

pub struct Mismatcher {
    pub mismatching_hash_same_filename: Vec<PathBuf>,
    pub mismatching_filename: Vec<PathBuf>,
    pub matching_hash_different_filename: Vec<PathBuf>
}

impl Mismatcher{
    #[allow(dead_code)]
    pub fn from_vectors(mismatching_hash_same_filename: Vec<PathBuf>, mismatching_filename: Vec<PathBuf>, matching_hash_different_filename: Vec<PathBuf>) -> Mismatcher {
        Mismatcher{mismatching_hash_same_filename, mismatching_filename, matching_hash_different_filename}
    }
    pub fn new() -> Mismatcher {
        Mismatcher{mismatching_hash_same_filename: Vec::new(), mismatching_filename: Vec::new(), matching_hash_different_filename: Vec::new()}
    }
    pub fn print_mismatches(&self) {
        let hash_different_name = self.matching_hash_different_filename.len() > 0;
        if hash_different_name {
            println!("Files with a matching hash but different filename:");
            for val in &self.matching_hash_different_filename {
                println!("{}", val.display());
            }
            println!("");
        }
        let total_mismatch = self.mismatching_filename.len() > 0;
        if total_mismatch {
            println!("Files that do not match in name or hash:");
            for val in &self.mismatching_filename {
                println!("{}", val.display());
            }
            println!("");
        }
        let mismatch_hash_only = self.mismatching_hash_same_filename.len() > 0;
        if mismatch_hash_only {
            println!("Files with a mismatching hash:");
            for val in &self.mismatching_hash_same_filename {
                println!("{}", val.display());
            }
            println!("");
        }
        if !hash_different_name && !total_mismatch && !mismatch_hash_only {
            println!("All files match");
        }
    }
}

pub fn compare_hashmap_to_directory(original: HashMap<PathBuf, Digest>, copy_path: &Path, config: &Config) -> Result<Mismatcher, &'static str> {
    let mut mismatcher = Mismatcher::new();
    let (copy_directory_map, _copy_directory_size) = generate_hashmap(&copy_path, &config.second_dir).map_err(|_e| "Error generating hashmap for file copy")?;
    for (key, value) in &copy_directory_map {
        if !original.contains_key(key) {
            let does_contain = &original.values().any(|val| compare_digest(val, value));
            if *does_contain {
                mismatcher.matching_hash_different_filename.push(key.clone());
            }
            else {
                mismatcher.mismatching_filename.push(key.clone());
            }
            continue;
        }
        let first_digest = match original.get(key) {
            Some(digest) => digest,
            None => return Err("Error finding hash for key")
        };
        let second_digest = match copy_directory_map.get(key) {
            Some(digest) => digest,
            None => return Err("Error finding hash for key")
        };
        if !compare_digest(first_digest, second_digest) {
            mismatcher.mismatching_hash_same_filename.push(key.clone());
        }
    }
    Ok(mismatcher)
}

fn compare_digest(first: &Digest, second: &Digest) -> bool{
    first.as_ref() == second.as_ref()
}