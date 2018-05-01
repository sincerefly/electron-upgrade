extern crate walkdir;
extern crate crypto;
extern crate zip;


use std::io::{Read, Write, Seek};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::fs;

use crypto::digest::Digest;
use crypto::md5::Md5;

use walkdir::{WalkDir, DirEntry};

use zip::write::FileOptions;
use zip::result::ZipError;


fn get_md5(file_path: &String) -> String {


    let mut buffer = Vec::new();
    let mut hasher = Md5::new();

    // let mut f = File::open("D:/1.jpg").unwrap();
    let mut f = File::open(file_path.to_owned()).unwrap();

    f.read_to_end(&mut buffer).unwrap();
    hasher.input(&buffer);

    hasher.result_str()
}

fn path_info(path: &String) -> (HashMap<String, String>, Vec<String>) {

    let mut file_list: Vec<String> = vec![];
    let mut dir_list: Vec<String> = vec![];

    let mut file_dict: HashMap<String, String> = HashMap::new();

    
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let path = String::from(entry_path.to_str().unwrap());
        
        if entry_path.is_dir() {
            dir_list.push(path);
        } else {
            file_list.push(path);
        }
    }

    println!("========== file ==========");

    let file_counter = file_list.len();
    for (i, path) in file_list.iter().enumerate() {
        let md5_str = get_md5(path);
        file_dict.insert(md5_str.clone(), path.clone());

        println!("({}/{}) File<{}>: {}", i+1, file_counter, md5_str, path);
    }

    println!("========== directory ==========");

    let dir_counter = dir_list.len();
    for (i, path) in dir_list.iter().enumerate() {
        println!("({}/{}) Dir: {}", i+1, dir_counter, path);
    }

    (file_dict, dir_list)
}

fn get_diff<'a>(current_info: &(HashMap<String, String>, Vec<String>), 
            latest_info: &(HashMap<String, String>, Vec<String>)) -> Vec<(&'a str, String)> {

    let &(ref c_file_dict, ref c_dir_list) = current_info;
    let &(ref l_file_dict, ref l_dir_list) = latest_info;
    let mut need_pack: Vec<(&str, String)> = vec![];

    println!("========== add or update ==========");
    let current_wrap_dirname = c_dir_list.first().unwrap();
    let latest_wrap_dirname = l_dir_list.first().unwrap();

    for dirname in l_dir_list {
        if !c_dir_list.contains(&dirname.replace(latest_wrap_dirname, current_wrap_dirname)) {
            need_pack.push(("dir", dirname.to_owned()));
            println!("=> newdir {}", dirname);
        }
    }
    for (key, value) in l_file_dict {
        if !c_file_dict.contains_key(key) {
            need_pack.push(("file", value.to_owned()));
            println!("=> upsert {} {}", key, value);
        }
    }
    need_pack
}


fn copy_source(sources: &Vec<(&str, String)>) {

    println!("========== collect ==========");

    let sources_first = &sources.first().clone().unwrap().1;
    let res: Vec<&str> = sources_first.split("\\").collect();
    let latest_wrap_dirname = res[0];
    
    for p in sources {
        if p.0 == "dir" {
            let need_create = p.1.replace(latest_wrap_dirname, "__updater");
            println!("=> create dir {}", &need_create);
            fs::create_dir_all(&need_create);
        }
    }
    for p in sources {
        if p.0 == "file" {
            let new_file = &p.1.replace(latest_wrap_dirname, "__updater");
            let new_file = Path::new(&new_file);
            let new_file_path = new_file.parent().unwrap();
            println!("=> create dir: {:?}", new_file_path.display());
            fs::create_dir_all(&new_file_path);
        }
    } 
    for p in sources {
        if p.0 == "file" {
            let new_file = p.1.replace(latest_wrap_dirname, "__updater");
            println!("=> copy file: {} to {}", p.1, &new_file);
            fs::copy(&p.1, new_file);
        }
    } 
}

fn pack_it() {

    println!("========== pack ==========");

    let src_dir = "__updater";
    let dst_file = "updater.zip";

    match doit(src_dir, dst_file, zip::CompressionMethod::Deflated) {
        Ok(_) => println!("=> done: {} written to {}", src_dir, dst_file),
        Err(e) => println!("Error: {:?}", e),
    }

}

fn zip_dir<T>(it: &mut Iterator<Item=DirEntry>, prefix: &str, writer: T, method: zip::CompressionMethod)
              -> zip::result::ZipResult<()>
    where T: Write+Seek
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix))
            .unwrap()
            .to_str()
            .unwrap();

        if path.is_file() {
            println!("=> adding {:?} as {:?} ...", path, name);
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(src_dir: &str, dst_file: &str, method: zip::CompressionMethod) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

fn main() {
    
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <old_directory> <new_directory>",
                 args[0]);
        return 1;
    }

    let src_dir = &*args[1];
    let dst_file = &*args[2];

    let e = time::SystemTime::now();

    // let path = String::from("data_current");
    let path = String::from("win-ia32-unpacked");
    let current_info = path_info(&path);

    // let path = String::from("data_latest");
    let path = String::from("win-ia32-unpacked_new");
    let latest_info = path_info(&path);

    let sources = get_diff(&current_info, &latest_info);

    copy_source(&sources);

    pack_it();

    let ed = time::SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());

}



