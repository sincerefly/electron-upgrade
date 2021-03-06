extern crate zip;
extern crate walkdir;

use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use zip::write::FileOptions;
use zip::result::ZipError;

use walkdir::{WalkDir, DirEntry};
use std::path::Path;
use std::fs::File;

fn main() {
    std::process::exit(real_main());
}

#[cfg(feature = "flate2")]
const METHOD_DEFLATED : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(feature = "flate2"))]
const METHOD_DEFLATED : Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2 : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2 : Option<zip::CompressionMethod> = None;

#[cfg(not(any(feature = "flate2", feature = "bzip2")))]
const METHOD_STORED : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);
#[cfg(any(feature = "flate2", feature = "bzip2"))]
const METHOD_STORED : Option<zip::CompressionMethod> = None;



fn real_main() -> i32 {
    
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <source_directory> <destination_zipfile>",
                 args[0]);
        return 1;
    }

    let src_dir = &*args[1];
    let dst_file = &*args[2];

    // let src_dir = "updater";
    // let dst_file = "updater.zip";

    for &method in [METHOD_BZIP2, METHOD_DEFLATED, METHOD_STORED].iter() {
        if method.is_none() { continue }
        match doit(src_dir, dst_file, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", src_dir, dst_file),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    return 0;
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
            println!("adding {:?} as {:?} ...", path, name);
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

























// extern crate zip;

// use std::fs::File;
// use std::io::{Seek, Write};
// use zip::result::ZipResult;
// use zip::write::{FileOptions, ZipWriter};

// static FILE_CONTENTS: &'static [u8] = include_bytes!("../../Cargo.lock");

// fn create_zip_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
//     let mut writer = ZipWriter::new(buf);
//     writer.start_file("example.txt", FileOptions::default())?;
//     println!("{:?}", FILE_CONTENTS);
//     writer.write(FILE_CONTENTS)?;
//     writer.finish()?;
//     Ok(())
// }


// fn main() {
//     let mut file = File::create("example.zip").expect("Couldn't create file");
//     create_zip_archive(&mut file).expect("Couldn't create archive");
// }