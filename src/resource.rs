use std::io::prelude::Read;
use std::fs::File;

pub fn load_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
