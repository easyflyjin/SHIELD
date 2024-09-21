use std::fs::{self, File};
use std::ops::IndexMut;
use std::path::Path;
use std::io::{self, Write, Read};
use crate::machine_hiding::file_system_operation::file_basic::FileStruct;
use crate::machine_hiding::{os_detection,file_system_operation::file_basic, file_log};

struct Commit{
    commit_hash: String
}

pub struct RootNode {
    root_node_id: String
}

pub struct FileNode {
    node_id: String,
    file_path: String,
}

impl FileNode {
    pub fn get_list(root_node: RootNode) -> Vec<Self> {
        let mut return_list: Vec<Self> = Vec::new();
        let root_file_name = ".shield/objects/".to_string() + &root_node.get_root_id().to_string();
        // println!("root_file_name {}", root_file_name );
        let mut root_file_content = FileStruct::new(root_file_name).read();
        // println!("root_file_content {}", root_file_content);
        let mut root_file_lines: Vec<&str> = root_file_content.lines().collect();
        // print!("root_file_lines {} {}", root_file_lines.get(0).unwrap(), root_file_lines.get(1).unwrap());
        // root_file_lines.into_iter().map(|line| {

        // });

        for line in root_file_lines {
            let line_content: Vec<&str> = line.split_whitespace().collect();
            let node_id = line_content.get(0).unwrap();
            let file_path = line_content.get(1).unwrap();
            // println!("node_id {}", node_id);
            // println!("file_path {}", file_path);
            return_list.push(FileNode { node_id: node_id.to_string(), file_path: file_path.to_string() })
        }

        return return_list;
    }

    pub fn get_file_path(&self) -> &String {
        return &self.file_path;
    }

    pub fn get_node_id(&self) -> &String {
        return &self.node_id;
    }
}

impl Commit {
    pub fn new() -> Self{
        let name = "COMMIT".to_string();
        Commit {
            commit_hash: file_log::generate_hash_id(&name) 
        }
    }

    pub fn get_commit_id(&self) -> &String {
       return  &self.commit_hash;
    }
}

impl RootNode {
    pub fn new() -> Self{
        let name: String = "ROOTNODE".to_string();
        RootNode { 
            root_node_id: file_log::generate_hash_id(&name) 
        }
    }

    pub fn existing(node_id: String) -> Self{
        let name: String = "ROOTNODE".to_string();
        RootNode { 
            root_node_id: node_id
        }
    }

    pub fn get_root_id(&self) -> &String{
        return &self.root_node_id;
    }
}


fn read_current_head() -> String {
    let head_path = FileStruct::new(".shield/HEAD".to_string());
    return head_path.read();
}

fn branch( repo_path: &str,branch_name: &str, user_id: &str) -> io::Result<()> {
    let head_commit_path = read_current_head();
    println!("head_coomit_path {}", head_commit_path);
    let head_commit_id = FileStruct::new(".shield/".to_string() + &head_commit_path).read();
    println!("head_commit_id {}", head_commit_id);


    // let _ = file_basic::create_folder(".shield");
    // Create new files for the branch
    println!("{}", format!(".shield/logs/refs/heads/{}", branch_name));
    println!("{}", format!(".shield/refs/heads/{}", branch_name));
    let logs_path = FileStruct::new(format!(".shield/logs/refs/heads/{}", branch_name));
    let refs_path = FileStruct::new(format!(".shield/refs/heads/{}", branch_name));
    // let logs_path = Path::new(repo_path).join(format!(".shield/logs/refs/HEAD/{}", branch_name));
    // let refs_path = Path::new(repo_path).join(format!(".shield/refs/HEAD/{}", branch_name));

    // Write to LOGS
    logs_path.create_file();
    let logs_content = "0000000000000000000000000000000000000000   ".to_string() + &head_commit_id;
    logs_path.write(&logs_content);
    // let mut logs_file = fs::File::create(logs_path)?;
    // writeln!(logs_file, "0 {}\n{} {}", head_commit_id, head_commit_id, user_id)?;

    // Write to REFS
    refs_path.create_file();
    refs_path.write(&head_commit_id);
    // let mut refs_file = fs::File::create(refs_path)?;
    // writeln!(refs_file, "{}", head_commit_id)?;

    Ok(())
}

pub fn branch_main(args:Vec<&str>) {
    if args.len()<=2{
        println!("please enter a branch name");
    }else if args.len()>3{
        println!("No space in a branch name, or you can add double quotes on the branch name");
    }else{   
        let branch_name = args[2];
        let user_id = "user123";
        let repo_path=os_detection::pwd();
        let is_repo = file_basic::folder_is_exist(".shield");

        if is_repo {

            match branch( &repo_path,branch_name, user_id) {
                Ok(_) => println!("Branch '{}' created.", branch_name),
                Err(e) => println!("Failed to create branch: {}", e),
            }

        }
        else {
            println!("Not a shield repository"); 
        }
    }
}

pub fn commit_files(){
    let new_commit: Commit = Commit::new();
    let root_node_of_tree: RootNode = RootNode::new();
    
    if is_first_commit() {
        let mut f_master = file_basic::FileStruct::new(".shield/refs/heads/master".to_string());
        let mut f_master_logs = file_basic::FileStruct::new(".shield/logs/refs/heads/master".to_string());
        let mut f_commit_file = file_basic::FileStruct::new(".shield/objects/".to_string() + new_commit.get_commit_id());
        let mut f_root_file = file_basic::FileStruct::new(".shield/objects/".to_string() + root_node_of_tree.get_root_id());
        //println!("{}", f_root_file.file_name);
        let mut f_index = file_basic::FileStruct::new(".shield/index".to_string());
       // println!("{}",&f_index.file_name);
        let index_file_content = f_index.read();
        let master_log_content = "0000000000000000000000000000000000000000 ".to_string() + new_commit.get_commit_id();

        f_master.create_file();
        f_master_logs.create_file();
        f_commit_file.create_file();
        f_root_file.create_file();

        f_master_logs.write(&master_log_content[..]);
        f_master.write(new_commit.get_commit_id());
        f_commit_file.write(&root_node_of_tree.get_root_id());
        f_root_file.write(&index_file_content[..]);

        println!("{}", &index_file_content);

        f_index.remove();
    }
    else {
        let head_file_comtent = FileStruct::new(".shield/HEAD".to_string()).read();
        println!("head_file_comtent {}", head_file_comtent);
        let mut f_master = file_basic::FileStruct::new(".shield/".to_string() + &head_file_comtent);
        let mut f_master_logs = file_basic::FileStruct::new(".shield/logs/".to_string() + &head_file_comtent);
        let mut f_commit_file = file_basic::FileStruct::new(".shield/objects/".to_string() + new_commit.get_commit_id());
        let mut f_root_file = file_basic::FileStruct::new(".shield/objects/".to_string() + root_node_of_tree.get_root_id());
        //println!("{}", f_root_file.file_name);
        let mut f_index = file_basic::FileStruct::new(".shield/index".to_string());
        //println!("{}",&f_index.file_name);
        println!("reading index");
        let index_file_content = f_index.read();
        let master_log_content = "\n".to_string() + &f_master.read() +" "+ new_commit.get_commit_id();
        //println!("Master_log_content:");
        println!("master_log_content {}",&master_log_content);
        //println!("end_content:");
        
        f_master.remove();
        f_master.create_file();
        f_commit_file.create_file();
        f_root_file.create_file();

        f_master_logs.write(&master_log_content[..]);
        f_master.write(new_commit.get_commit_id());
        f_commit_file.write(&root_node_of_tree.get_root_id());
        f_root_file.write(&index_file_content[..]);

        println!("{}", &index_file_content);

        f_index.remove();

    }

}

pub fn add_files(){
    let pwd = os_detection::pwd();
    let is_repo = file_basic::folder_is_exist(".shield");
    let mut files_list: Vec<FileStruct> = file_basic::get_file_list();


    if is_repo {
        //  if (is_first_commit()) { 
        //     add_file_hash(List<File>)
        // }
        // else{
        //     compare_all_files_with_last_commit()
        //     add_file_hash(List<Files>);
        // }

        // THIS STATEMENT SHOULD BE OUTSIDE ITERATOR
        let index_file = FileStruct::new(".shield/index".to_string());
        index_file.create_file();

        if is_first_commit()  {
            files_list.iter().for_each(|file| {
                let content = file.read();
                //println!("{}", &content);
                let mut hash = file_log::generate_hash_id(file.get_file_name());
                //println!("{}", file.get_file_name());
                let new_file_name = ".shield/objects/".to_string() + &hash;
                let f = file_basic::FileStruct::new(new_file_name);
                f.create_file();
                f.write(&content);
                hash = hash + " "+ &file.file_name+"\n";
                index_file.write(&hash);
            });
        }
        else{
            //compare_all_files(&mut files_list);
            files_list.iter().for_each(|file| {
                let content = file.read();
                println!("{}", &content);
                let mut hash = file_log::generate_hash_id(file.get_file_name());
                println!("{}", file.get_file_name());
                let new_file_name = ".shield/objects/".to_string() + &hash;
                let f = file_basic::FileStruct::new(new_file_name);
                f.create_file();
                f.write(&content);
                hash = hash + " "+ &file.file_name+"\n";
                index_file.write(&hash);
            });
        }
        // TODO: PUT THESE IN AN ITERATOR OVER files_list
        // ITERATOR ENDS HERE
    }
    else{
        println!("Not a shield repository");
    }
}

fn add_file_hash(){
    // for each item in list
    // let hash_id = machine_hash
    // machine.createfile(hash_id)
    // contents = read_from_durrent_item
    // manchine.add_contents_to_file(hash_id, contents)
}

fn is_first_commit() -> bool{
    let master_file: FileStruct = FileStruct::new(".shield/refs/heads/master".to_string());
    return !master_file.file_is_exist();
}

fn compare_all_files(file_list: &mut Vec<FileStruct>) {
    //get a list of all files in last commit
    //for each file in file_list
        //if file's contents exactly match last commit's contents
            // add the file in ignore_list
        // else
            // continue
    // for-end

    // file_list = file_list - ignore_list
}

pub fn checkout(args:Vec<&str>) {
    if args.len()!=3 {
        println!("Invalid checkout command syntax");
    }
    else {
        // if branch exists
            // change the refs/HEAD file
            // get the latest commit ID from refs
            // delete all files from pwd
            // re-create them from commit_id -> root_node -> list_of_files
        // else 
            // no checkout
        
        let branch_name = args[2];
        println!("branch name {}", branch_name);

        if (branch_exists(branch_name)) {
            let head_file = FileStruct::new(".shield/HEAD".to_string());
            head_file.remove();
            head_file.create_file();
            let head_file_content = "refs/heads/".to_string() + &branch_name;
            println!("head_file_content {}", head_file_content);

            head_file.write(&head_file_content);
            let head_commit = FileStruct::new(".shield/".to_string() + &head_file_content).read();
            println!("head_commit {}", head_commit);
            
            let file_list = file_basic::get_file_list();
            file_list.into_iter().map(|x| x.remove());

            // recreate
            let head_commit_file_path = ".shield/objects/".to_string() + head_commit.as_str();
            println!("head_commit_file_path {}", head_commit_file_path);
            let rootnode = RootNode::existing(FileStruct::new(head_commit_file_path).read());
            println!("rootnode {}", rootnode.get_root_id());

            let file_node_list = FileNode::get_list(rootnode);
            let _: Vec<_> = file_node_list.iter().map(|file_node| {
                println!("file_node.file_path.to_string() {}" ,file_node.file_path.to_string());
                let replace_file = FileStruct::new(".".to_string() + &file_node.file_path);
                replace_file.create_file();

                let path = ".shield/objects/".to_string() + &file_node.node_id;
                println!("path {}", path);
                let content_of_replace_file = &FileStruct::new(path).read();
                println!("content_of_replace_file {}", content_of_replace_file);
                replace_file.write(content_of_replace_file);
            }).collect();
        }

    }   
}

pub fn branch_exists(branch_name: &str) -> bool {
    let branch_file_name = ".shield/refs/heads/".to_string()+ branch_name;
    let branch_file = FileStruct::new(branch_file_name);
    println!("exists {}", branch_name);
    return branch_file.file_is_exist(); 
}
