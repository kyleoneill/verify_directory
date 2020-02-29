use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use super::Config;

use super::gen_hashmap::generate_hashmap;

extern crate data_encoding;
extern crate ring;

use ring::digest::Digest;

pub fn compare_hashmap_to_directory(original: HashMap<PathBuf, Digest>, copy_path: &Path, config: &Config) -> Result<(), &'static str> {
    let mut problems = 0;
    let copy_directory_map = generate_hashmap(&copy_path, &config.second_dir).map_err(|_e| "Error generating hashmap for file copy")?;
    for (key, _value) in &copy_directory_map {
        if !original.contains_key(key) {
            println!("Original is missing file with path: {}", key.display());
            problems += 1;
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
            println!("Mismatching hash for file: {}", key.display());
            problems += 1;
        }
    }
    println!("Verification finished with {} issues", problems);
    Ok(())
}

fn compare_digest(first: &Digest, second: &Digest) -> bool{
    first.as_ref() == second.as_ref()
}