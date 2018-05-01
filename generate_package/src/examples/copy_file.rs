use std::fs;
use std::path::Path;
use std::ffi::OsStr;

fn copy_source(sources: &Vec<(&str, String)>) {

    let sources_first = &sources.first().clone().unwrap().1;
    let res: Vec<&str> = sources_first.split("\\").collect();
    let latest_wrap_dirname = res[0];

    for p in sources {
        if p.0 == "dir" {
            let need_create = p.1.replace(latest_wrap_dirname, "__updater");
            println!("create dir: {:?}", &need_create);
            fs::create_dir_all(&need_create);
        }
    } 

    for p in sources {
        if p.0 == "file" {
            let new_file = Path::new(&p.1);
            let new_file_path = new_file.parent().unwrap();
            println!("create dir: {:?}", new_file_path);
            fs::create_dir_all(&new_file_path);
        }
    }  

    for p in sources {
        if p.0 == "file" {
            let new_file = p.1.replace(latest_wrap_dirname, "__updater");
            println!("copy file: {} to {}", p.1, &new_file);
            fs::copy(&p.1, new_file);
        }
    }  
}


fn collect_source<'a>() -> Vec<(&'a str, String)> {

    let sources = vec![
        ("file", String::from("data_latest\\newfile.txt")),
        ("file", String::from("data_latest\\resources\\resources.asar")),
        ("file", String::from("data_latest\\newdir\\123.xls")),
        ("dir", String::from("data_latest\\newdir"))
    ];

    sources
}

fn main() {
    let sources = collect_source();
    copy_source(&sources);
}



