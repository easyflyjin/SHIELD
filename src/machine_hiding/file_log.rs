
use sha1::{Sha1, Digest};
use std::time::{SystemTime, UNIX_EPOCH};


use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};
use crate::behaviour_hiding::output;
use crate::{machine_hiding::file_system_operation::file_basic::{self, FileStruct}};

pub fn generate_hash_id(filename: &String) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.as_secs();

    let mut hasher = Sha1::new();

    hasher.update(time.to_be_bytes());
    hasher.update(filename);

    let hash = hasher.finalize();
    return format!("{:x}", hash)
}

pub fn pull(from: &str, to: &str) -> io::Result<()> {

    let from_dir=Path::new(from);
    let to_dir=Path::new(to);

    if !from_dir.is_dir() {
        return Err(io::Error::new(ErrorKind::NotFound, "Source directory does not exist"));
    }
    if !to_dir.is_dir() {
        fs::create_dir_all(to_dir)?;
    }
    fs::read_dir(from_dir)?
        .filter_map(Result::ok)  
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .try_for_each(|path| {
            let dest_path = to_dir.join(path.file_name().ok_or_else(|| io::Error::new(ErrorKind::Other, "Missing file name"))?);
            fs::copy(&path, &dest_path)?;
            Ok(())
        })
}


pub fn log() {
    // get current branch from .shield/HEAD
    // get the contents from .sheld/logs/<>
    // split with respect to \n
    // print the second part from each split

    let head_file_comtent = FileStruct::new(".shield/HEAD".to_string()).read();
    let mut f_logs = file_basic::FileStruct::new(".shield/logs/".to_string() + &head_file_comtent).read();

    let lines: Vec<&str> = f_logs.lines().collect();

    lines.into_iter().map(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        output::print_message(words.get(1).unwrap());
    }).collect()
}



// #[derive(Debug, Clone)]
// pub struct Log {
//     file_path: String,
//     timestamp: u64,
//     version_id: String,
//     modification_content: String,
// }

// impl Log {
//     pub fn new(file_path: String, modification_content: String) -> Log {
//         let timestamp = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards")
//             .as_secs();

//         let version_id = format!("{}-{}", file_path, timestamp); 

//         Log {
//             file_path,
//             timestamp,
//             version_id,
//             modification_content,
//         }
//     }
// }


// pub fn cat() {}

// pub fn retrieve() {
//     // This function gets contents of a file at a partivular version.
// }
