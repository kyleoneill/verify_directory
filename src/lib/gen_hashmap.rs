use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

extern crate data_encoding;
extern crate ring;

use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn generate_hashmap<'a>(dir: &Path, prefix_strip: &PathBuf) -> Result<HashMap<PathBuf, Digest>, std::io::Error> {
    let mut directory_map = HashMap::new();
    if dir.is_dir() {
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let sub_dir_map = generate_hashmap(&path, &prefix_strip)?;
                directory_map.extend(sub_dir_map);
            }
            else {
                let file_digest = hash_file(&path)?;
                let file_path = path.strip_prefix(prefix_strip).unwrap().to_path_buf();
                directory_map.insert(file_path, file_digest);
            }
        }
    }
    Ok(directory_map)
}

fn hash_file(file_name: &Path) -> Result<Digest, std::io::Error> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    Ok(digest)
}


fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}