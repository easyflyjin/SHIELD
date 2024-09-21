use crate::behaviour_hiding::output;
use crate::machine_hiding::file_log;
use crate::machine_hiding::{os_detection,file_system_operation::file_basic};
use crate::repository_hiding;
use crate::repository_hiding::{repository_origin,repository_local::repository_versioning, repository_local::merge_conflict};

use shellwords;

use std::{
    // env::args,
    io::{self, Write},
};

pub fn initialization() {
    output::print_welcome();
    loop {
        let mut user_input = String::new();

        print!("dvcs>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();

        let user_input = user_input.trim_end();
        let user_input = user_input.trim_start();

        if user_input == "exit" {
            break;
        }

        // divide the input message to a vec, each element represents a word, for exampele, user input = shield add, args = ["shield", "add"].
        if !user_input.contains(' ') {
            let mut args = Vec::new();
            args.push(user_input);
            //println!("{:?}", args);
            divide_command(args);
        } else {
            let args = match shellwords::split(user_input) {
                Ok(args) => args,
                Err(err) => {
                    eprintln!("Error:{}", err);
                    return;
                }
            };
            let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            //println!("{:?}", args);
            divide_command(args);
        }
    }
}

fn divide_command(args: Vec<&str>) {
    if !args.is_empty() {
        if args[0] != "shield" {
            println!("{} is not a valid shield command, please type shield help if you have any questions",args[0]);
        } else if args.len() == 1 {
            println!("please tell us what's your command, if you don't know, type \"shield help\" to get some help");
        } else {
            // TODO: put the commands and description in a map as an improvement
            match args[1]{
                        "help" =>  output::print_help(),
                        "quit" =>  std::process::exit(0),
                        "init" => repository_origin::init_main(),
                        //"pwd" =>  println!("{}",os_detection::pwd()),
                        "createfile"=> process_create(args),
                        "createfolder" => process_create_folder(args),
                        "branch" => repository_versioning::branch_main(args),
                        "cd" => process_cd(args),
                        "write" => process_write(args), // only for testing
                        "ls" => process_ls(),
                        "cat" =>process_read(args),
                        "rm" => process_remove(args),
                        "mv" =>process_mv(args),
                        "remove_folder" => process_remove_folder(args),
                        "folder_is_exist" => process_is_exist(args),
                        "file_is_exist" => process_file_is_exist(args),
                        //"file_list" => process_file_list(args),
                        "add" => repository_versioning::add_files(),
                        "commit" => repository_versioning::commit_files(),
                        "clone" => process_clone(args),
                        "merge" => repository_hiding::repository_local::merge_conflict::merge(args),
                        "checkout" => repository_hiding::repository_local::repository_versioning::checkout(args),
                        "heads" => process_heads(args),
                        "log" => file_log::log(),
                        "diff" => repository_hiding::repository_local::merge_conflict::diff(args),
                        "push" => process_push(args),
                        "pull" => process_pull(args),
                        _ => println!("{} is not a valid shield command, please type shield help if you have any questions",args[1])
                    }
        }
    }
}

fn process_create(args:Vec<&str>){
    if args.len()<=2{
        println!("please enter a file name");
    }else if args.len()>3{
        println!("No space in a file name, if you insist, you can add double quotes on the file name");
    }else{    
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        let _ = f.create_file();
    }
}

fn process_create_folder(args:Vec<&str>){
    if args.len()<=2{
        println!("please enter a folder name");
    }else if args.len()>3{
        println!("No space in a folder name, use slash \"/\" to separate the folder name");
    }else{    
        //let mut f = file_basic::FileStruct::new();
        let _ = file_basic::create_folder(args[2]);
    }
}

fn process_cd(args:Vec<&str>){
    if args.len()<=2 {
        let _ = output::print_message(os_detection::pwd().as_str());
    }else{
        let _ = os_detection::pwd_move(args[2]);
        let _ =  output::print_message(os_detection::pwd().as_str());
    }
}

fn process_write(args:Vec<&str>){
    if args.len()<=2{
        output::print_message("please enter your file name and the content you want to add");
    }else if args.len()==3{
        output::print_message("please enter your content");
    }else{
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        let _ = f.write(args[3]);
    }
}

fn process_ls(){
    let _ = os_detection::ls();
}
fn process_read(args:Vec<&str>){
    if args.len()<=2{
        output::print_message("Please input the file name that you want to read");
    }else if args.len()>3{
        println!("No space in a file name, if you insist, you can add double quotes on the file name");
    }else{
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        let fileread = f.read();
        output::print_message(fileread.as_str());
    }
}

fn process_remove(args: Vec<&str>){
    if args.len()<=2{
        output::print_message("Please input the file name that you want to remove");
    }else if args.len()>3{
        println!("No space in a file name, if you insist, you can add double quotes on the file name");
    }else{
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        let _ =f.remove();
    }
}

fn process_mv(args: Vec<&str>){
    if args.len() <= 2{
        output::print_message("Please input the file name and target address(relative) that you want to move");
    }else if args.len() == 3{
        output::print_message("Please input the target address(relative) that you want to move");
    }else if args.len() > 4{
        println!("No space in a file name, if you insist, you can add double quotes on the file name");
    }else{
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        let _ = f.mv(args[3]);
    }
}

fn process_remove_folder(args:Vec<&str>){
    if args.len()<=2{
        println!("please enter a folder name");
    }else if args.len()>3{
        println!("No space in a folder name, use slash \"/\" to separate the folder name");
    }else{    
        //let mut f = file_basic::FileStruct::new();
        let _ = file_basic::remove_folder(args[2]);
    }
}

fn process_is_exist(args: Vec<&str>){
    if args.len()<= 2{
        println!("please enter a folder name");
    }else if args.len()>3{
        println!("No space in a folder name, use slash \"/\" to separate the folder name");
    }else{
        let is_exist =file_basic::folder_is_exist(args[2]);
        println!("{}",is_exist);
    }
}

fn process_file_is_exist(args: Vec<&str>){
    if args.len()<= 2{
        println!("please enter a file name");
    }else if args.len()>3{
        println!("No space in a file name, use slash \"/\" to separate the folder name");
    }else{
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        println!("{}",f.file_is_exist());
    }
}

// fn process_file_list(args: Vec<&str>){
//     if args.len() > 2{
//         println!("No need other arguments");
//     }else{
//         let s = file_basic::get_file_list();

//         println!("{:?}", s);

//     }
// }

fn process_clone(args: Vec<&str>){
    if args.len() <= 2{
        println!("Please enter the repository you want to clone");
    }else if args.len()>3{
        println!("No space in a path, if you want to add space, please double quote your path!");
    }else{
        if let Err(e) = file_basic::clone(args[2]) {
            println!("Failed to clone the repository: {}", e);
        }

        
    }
}
fn process_heads(args: Vec<&str>){
    if args.len() > 2{
        println!("Heads does not need additional arguments!");
    }else{
        let fl = file_basic::heads().join(" ");
        println!("{}",fl);
    }
}
fn process_pull(args: Vec<&str>){
    if args.len()>3{
        output::print_message("No space in the path, if you insist, please add double quote on the path");
    }else if args.len()<=2{
        output::print_message("Please enter which repo you want to pull");
    }else{
        repository_origin::push(args[2],&os_detection::pwd());
    }
}
fn process_push(args: Vec<&str>){
    if args.len()>3{
        output::print_message("No space in the path, if you insist, please add double quote on the path");
    }else if args.len()<=2{
        output::print_message("Please enter which repo you want to push");
    }else{
        repository_origin::push(&os_detection::pwd(),args[2]);
    }
}