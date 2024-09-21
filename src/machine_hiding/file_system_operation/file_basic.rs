//use crate::permission;
use crate::machine_hiding::os_detection;
use std::fs;
//use std::io::Result;
use std::fs::File;
use std::io::{self, Write};
use walkdir::WalkDir;
use std::path::{PathBuf, Path};
use crate::behaviour_hiding::output;
use crate::machine_hiding::file_system_operation::file_permission;
use std::fs::OpenOptions;
//use std::io::prelude::*;
// TODO: create a trait
pub struct FileStruct {
    pub file_name: String,
    perm:file_permission::Permission,
    cwd: String
}

impl FileStruct {
    pub fn new(file_name: String) -> FileStruct {
        let cwd = os_detection::pwd();
        
        let perm = file_permission::Permission{
            readable: true,
            writable: true,
        };
        //println!("{} {}",perm.writable,perm.readable);
        FileStruct { file_name, perm, cwd }
    }

    pub fn get_file_name(&self) -> &String{
        return &self.file_name;
    }

    pub fn create_file(&self) -> io::Result<()> {
        let mut filepath = PathBuf::from(&self.cwd);

        filepath.push(&self.file_name);

    
        match File::create(&filepath) {
            Ok(_) =>{} //output::print_message("File created successfully"),
            Err(e) => {
                output::print_message("Failed to create file");
                println!("{}",e);
                return Err(e);
            }
        }
    
        Ok(())
    }
    pub fn write(&self, content:&str) -> std::io::Result<()>  {
        if self.perm.writable == true{
            let mut w = OpenOptions::new().append(true).open(&self.file_name)?;
            let c = content.to_string();
            w.write_all(c.as_str().as_bytes())?;
        }else{
            output::print_message("The file cannot be written, you have to acquire permission first.");
        }
        Ok(())
    }
    
    pub fn read(&self) ->String {
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();

        if self.perm.readable == true{
            return fs::read_to_string(fpr).unwrap_or_else(|err| {
                "Failed to Read, please check the file is exist or not".to_string()
            });
        }else{  
            return "The file cannot be read, you have to acquire permission first.".to_string();
        }
    }
    
    pub fn remove(&self)-> io::Result<()>{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();

        match fs::remove_file(fpr) {
            Ok(_) =>println!("{} has been successfully removed!",self.file_name),
            Err(e) => {}
        }
        Ok(())    
    }

    pub fn mv(&self, target:&str) -> io::Result<()>{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();
        
        let abs_target = self.cwd.clone()+"/"+target+"/"+self.file_name.clone().as_str(); 
        println!("{}",fpr);
        match fs::rename(fpr,abs_target){
            Ok(_)=> println!("{} has been successfully moved!", self.file_name),
            Err(e) => println!("Failed to move the file, please check the file name and the target path!")
        }
        Ok(())
    }
    pub fn file_is_exist(&self)->bool{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();
        let path = PathBuf::from(fpr);

        if path.exists() {
            return true;
        } else {
            return false;
        }
    }
}

// create_folder and create_file should be in one method of function
pub fn create_folder(create_dir: &str) -> std::io::Result<()> {
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(create_dir);

    match fs::create_dir_all(&path) {
        Ok(_) => {
            //println!("Folder created successfully.");
            Ok(())
        },
        Err(e) => {
            println!("Failed to create folder");
            //Err(e)
            Ok(())
        }
    }
}

pub fn remove_folder(remove_dir:&str)->std::io::Result<()>{
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(remove_dir);

    match fs::remove_dir_all(&path) {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            println!("Failed to remove folder");
            //Err(e)
            Ok(())
        }
    }
}

pub fn folder_is_exist(folder_name:&str)->bool{
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(folder_name);
    path.exists() && path.is_dir()
}

pub fn get_file_list()->Vec<FileStruct>{
    let cwd = os_detection::pwd();
    let file_paths: Vec<String> = WalkDir::new(cwd.clone())
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect();
    let files_list: Vec<FileStruct> = file_paths.iter()
    .filter(|path| (!path.contains(".shield") && !path.contains(".DS_Store")))
    .map(|path| {
        let modified_path = path.to_string().replace(&cwd, "");
        FileStruct::new(modified_path)})
    .collect();
    //FileStruct::new(new_file_name);
    // for file in &files_list {
        //     println!("{}", file.file_name);
    // }
    return files_list;
}

pub fn clone(target_dir:&str)-> std::io::Result<()>{
    let cwd: String = os_detection::pwd();
    let cwd_path = Path::new(cwd.as_str());
    let td = Path::new(target_dir);
    let td_shield = Path::new(target_dir).join(".shield");//full path of a .shield folder in target path
    if td_shield.exists() && td_shield.is_dir() {
        match fs::read_dir(cwd_path) {
        Ok(mut entries) => {
            if entries.next().is_none() {
                for entry in WalkDir::new(target_dir) {
                    let entry = entry?;
                    let path = entry.path();
                    let relative_path = path.strip_prefix(target_dir).unwrap();
                    let target_path = cwd_path.join(relative_path);
            
                    if path.is_dir() {
                        fs::create_dir_all(&target_path)?;
                    } else {
                        fs::copy(path, &target_path)?;
                    }
                }          
                println!("The repository is successfully cloned!");    
            } else {
                println!("The directory is not empty. Failed to clone!");
            }
        }
        Err(e) => {
            println!("Failed to read the directory: {}", e);
        }
    }
    } else {
        println!("This is not a shield repository, please enter shield init to initialize a shield repo first!");
    }
    Ok(())
}
pub fn heads()->Vec<String>{
    let cwd: String = os_detection::pwd();

    let cwd_shield = Path::new(&cwd).join(".shield");//full path of a .shield folder in target path
    let mut file_names = Vec::new();
    let head_path = Path::new(&cwd).join(".shield").join("refs").join("heads");//full path of a .shield folder in target path


    if cwd_shield.exists() && cwd_shield.is_dir() {
        match fs::read_dir(head_path) {
        Ok(mut entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(name) = path.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    file_names.push(name_str.to_string());
                                }
                            }
                        }
                    }
                    Err(e) => println!("Error reading entry: {}", e)
                }
            }
        }
        Err(e) => {
            println!("Failed to read the directory: {}", e);
        }
    }
    } else {
        println!("This is not a shield repository, please enter shield init or shield clone to initialize a shield repo first!");
    }
    //println!("{:?}",file_names);
    return file_names;
}
