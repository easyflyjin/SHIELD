use std::fs::File;

use crate::{machine_hiding::file_system_operation::file_basic::{self, FileStruct}, repository_hiding::repository_local::repository_versioning::{self, RootNode}, behaviour_hiding::output};
use crate::repository_hiding::repository_local::repository_versioning::FileNode;

pub fn merge(args:Vec<&str>) {
    //check for right args
    //fetch head commit from current branch
    //fetch head commit from the argument branch
    //fetch list of file struct from current branch
    //fetch list of file struct from the argument branch
    
    //for each file in currentbranch do1
        // if filename is present in argument_file_struct
            // merge two files content
        // else
            // skip merging the file
    // end do1

    //for each file in argument branch
        // if file is not there in current file struct
            // add this file
    
    // maybe the user needs to do it by himself
    // shield add 
    // shield commit
    if args.len()!=3 {
        output::print_message("Invalid checkout command syntax");
        return;
    }

    let mut merge_branch_name = args[2];

    if (!repository_versioning::branch_exists(merge_branch_name)) {
        return;
    }

    let  current_branch_path = &FileStruct::new(".shield/HEAD".to_string()).read();
    // println!("current_branch_path {}",current_branch_path);
    let current_commit_id = FileStruct::new(".shield/".to_string() + current_branch_path).read();
    // println!("current_commit_id {}",current_commit_id);
    let  merge_branch_path = ".shield/refs/heads/".to_string() + merge_branch_name;
    // println!("merge_branch_path {}",merge_branch_path);
    let merge_commit_id = FileStruct::new(merge_branch_path).read();
    // println!("merge_commit_id {}",merge_commit_id);

    let current_branch_file_node_list = FileNode::get_list(RootNode::existing(
        FileStruct::new(".shield/objects/".to_string() + &current_commit_id).read()
    ));
    let merge_branch_file_node_list = FileNode::get_list(RootNode::existing(
        FileStruct::new(".shield/objects/".to_string() + &merge_commit_id).read()
    ));

    let __: Vec<_> = current_branch_file_node_list.iter().map(|file| {
        let _: Vec<_> = merge_branch_file_node_list.iter().map(|merge_file| {
            // println!("file.get_file_path() {}",file.get_file_path());
            // println!("merge_file.get_file_path() {}",merge_file.get_file_path());
            if (file.get_file_path() == merge_file.get_file_path()) {

                let mut f1 = FileStruct::new(file.get_file_path().to_string());
                let mut f2 = FileStruct::new(".shield/objects/".to_string() + merge_file.get_node_id());

                match merge_files(&f1, &f2) {
                    Ok(merged_content) => {
                        // println!("Delete and Writing down");
                       f1.remove();
                    //    println!("file.get_file_path().to_string() {}", file.get_file_path().to_string());
            
                        let mut f1_new = FileStruct::new(".".to_string() + file.get_file_path());
                        f1_new.create_file();
                        f1_new.write(&merged_content[..]);
                    }
                    Err(err) => {}
                }
            }
        }).collect();
    }).collect();

    let ___: Vec<_> = merge_branch_file_node_list.iter().map(|merge_file| {
        if(!file_exists_in_current(&current_branch_file_node_list, merge_file.get_file_path())) {
            let new_file = FileStruct::new(merge_file.get_file_path().to_string());
            let new_file_content = &FileStruct::new(".shield/objects/".to_string() + merge_file.get_node_id()).read();
            new_file.create_file();
            new_file.write(new_file_content);
        }
    }).collect();

    // let mut f1 = FileStruct::new(".\\file1.txt".to_string());
    // let mut f2 = FileStruct::new(".\\file2.txt".to_string());
    // match merge_files(&f1, &f2) {
    //     Ok(merged_content) => {
    //         println!("Delete and Writing down");
    //        f1.remove();

    //         let mut f1_new = FileStruct::new(".\\file1.txt".to_string());
    //         f1_new.create_file();
    //         f1_new.write(&merged_content[..]);
    //     }
    //     Err(err) => {}
    // }
}

fn merge_files(f1: &FileStruct, f2: &FileStruct) -> Result<String, std::io::Error> {
    let base_content = f1.read();
    let other_content = f2.read();
    let base_lines: Vec<&str> = base_content.lines().collect();
    let other_lines: Vec<&str> = other_content.lines().collect();
    let mut merged_content = String::new();

    let mut i = 0;
    let mut j = 0;

    while i < base_lines.len() || j < other_lines.len() {
        if i < base_lines.len() && j < other_lines.len() {
            if base_lines[i] == other_lines[j] {
                merged_content.push_str(base_lines[i]);
                merged_content.push('\n');
                i += 1;
                j += 1;
            } else {
                // Conflict occurred, handle it as needed
                merged_content.push_str("<<<<<<< Base\n");
                while i < base_lines.len() && base_lines[i] != other_lines[j] {
                    merged_content.push_str(base_lines[i]);
                    merged_content.push('\n');
                    i += 1;
                }
                merged_content.push_str("=======\n");
                while j < other_lines.len() && base_lines[i - 1] != other_lines[j] {
                    merged_content.push_str(other_lines[j]);
                    merged_content.push('\n');
                    j += 1;
                }
                merged_content.push_str(">>>>>>> Other\n");
            }
        } else if i < base_lines.len() {
            // Remaining lines in the base file
            while i < base_lines.len() {
                merged_content.push_str(base_lines[i]);
                merged_content.push('\n');
                i += 1;
            }
        } else if j < other_lines.len() {
            // Remaining lines in the other file
            while j < other_lines.len() {
                merged_content.push_str(other_lines[j]);
                merged_content.push('\n');
                j += 1;
            }
        }
    }

    // let changeset = Changeset::new(&base_content, &other_content, "");

    // let merged_content = match changeset {
    //     [Difference::Same(content)] => content.to_string(),
    //     [_, Difference::Same(content), _] => content.to_string(),
    //     [_, Difference::Same(content)] => content.to_string(),
    //     _ => {
    //         eprintln!("Conflict occurred while merging");
    //         base_content // For simplicity returning base_content
    //     }
    // };

    output::print_message(&merged_content);
    Ok(merged_content)
}

fn file_exists_in_current(current_branch_file_node_list: &Vec<FileNode>, merge_file_name: &String) -> bool {
    let mut result = false;
    let _: Vec<_> = current_branch_file_node_list.iter().map(|file| {
        if file.get_file_path() == merge_file_name {
            result = true;
        }
    }).collect();
    
    return result;

}

fn get_ref_id(current_branch_file_node_list: &Vec<FileNode>, file_name: &String) -> String {
    let mut result = String::new();
    let _: Vec<_> = current_branch_file_node_list.iter().map(|file| {
        // println!("ile.get_file_path() {}", file.get_file_path());
        // println!("file.name {}", file_name);
        if file.get_file_path() == file_name {
            result = file.get_node_id().to_string();
        }
    }).collect();
    
    return result;
}

pub fn diff(args: Vec<&str>) {

    if args.len()!=4 {
        output::print_message("Invalid diff command syntax");
        return;
    }
    let current_file_path = args[2].to_string();
    let compare_commit_id = args[3];

    // println!("current_file_path {}", current_file_path);
    // println!("compare_commit_id {}", compare_commit_id);

    let commit_node_list = FileNode::get_list(RootNode::existing(
        FileStruct::new(".shield/objects/".to_string() + &compare_commit_id).read()
    ));

    let ref_id = get_ref_id(&commit_node_list, &current_file_path);
    // println!("ref_id {}", ref_id);
    
    if ref_id.is_empty() {
        output::print_message("Seems like the ref id was not found");
        return;
    }

    let mut f1 = FileStruct::new(current_file_path.to_string());
    let mut f2 = FileStruct::new(".shield/objects/".to_string() + &ref_id);

    merge_files(&f1, &f2);
}