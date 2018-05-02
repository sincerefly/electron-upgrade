extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fs::File;
use std::io::Read;
use std::time;


fn get_md5(file_path: String) {

    let e = time::SystemTime::now();

    let mut buffer = Vec::new();
    let mut hasher = Md5::new();

    let mut f = File::open(file_path.to_owned()).unwrap();

    f.read_to_end(&mut buffer).unwrap();

    hasher.input(&buffer);
    println!("{}", hasher.result_str());
    let ed = time::SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());
}


fn main() {

    get_md5(String::from("Cargo.toml"));
    
}

