pub fn print_message(message: &str) {
    println!("{}", message);
}

pub fn print_help() {
    println!(
            "
These are shield commands commonly used:

- Initialize a new repository:   shield init
- Clone an existing repository:  shield clone <\"repo_url\">
- Check your current status:    shield status
- Add files to staging:         shield add
- Commit your changes:          shield commit <\"your message\">
- Push changes to remote:       shield push <\"target path\">
- Pull updates from remote:     shield pull <\"target path\">
- Remove a file from the directory:  shield rm <\"file_name\">
- Move a file to a different directory: shield mv <\"file_name\"> <\"new_path\">
- Check the files and folders:  shield ls
- Move to a specific directory      shield cd <\"folder_name\">
- Create a file in the repo:     shield createfile <\"file_name\">
- Create a folder in the repo:   shield createfolder <\"folder_name\">
- Write a line in a file:        shield write <\"file_name\"> <\"content\">
- Read a file content:           shield cat <\"file_name\"> 
- Check the diff of different commit:      shield diff <\"(backslash\\)file name\"> <\"commit id\">
- Check the heads:                shield heads
- Check the log:                  shield log
- Checkout to a different branch    shield checkout <\"branch name\">
- Switch to a different branch in the cloned repository:   shield branch <\"branch name\">
- Merge code from 2 branches:        shield merge  <\"branch_name\">

            "
    );
}

pub fn print_welcome() {
    print!(
"
Welcome to S.H.I.E.L.D - The Distributed Version Control System

S.H.I.E.L.D efficiently handles projects of any size with speed and accuracy.
It allows you to collaborate seamlessly with your team, no matter where you are.

Getting started:
- Initialize a new repository:   shield init
- Clone an existing repository:  shield clone <\"repo_url\">

For more commands and usage information, type 'shield help'.

Thank you for choosing S.H.I.E.L.D for your version control needs.
Happy coding!

(c) 2023 AVENGERS Team - CSC 253/453
"
    );
}
