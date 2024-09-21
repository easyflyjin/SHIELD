use std::env;
use std::fs;
use std::path::Path;
use std::io;
use colored::*;

pub fn detect_OS()->&'static str{
    std::env::consts::OS
}

pub fn pwd()->String{
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

pub fn pwd_move(target: &str) -> io::Result<()> {
    let current_dir = env::current_dir()?;

    let new_dir = match target {
        "." => current_dir,
        ".." => current_dir.parent().unwrap_or(&current_dir).to_path_buf(),
        _ => Path::new(target).to_path_buf(),
    };

    env::set_current_dir(&new_dir)?;
    Ok(())
}

pub fn ls() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let mut line = String::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            let name = if metadata.is_dir() {
                filename.blue().to_string()
            } else {
                filename.green().to_string()
            };
            line.push_str(&name);
            line.push(' '); 
            line.push(' '); 
            line.push(' '); 
        }
    }

    println!("{}", line);

    Ok(())
}