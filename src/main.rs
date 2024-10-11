use std::io;
use std::process::{exit, Command};
use names::Generator;
// use names::Generator;

fn main() {
    update_commit_push();
}

fn update_commit_push(){
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");
    if !add_command.status.success(){
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1);
    }
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");
    if !commit_command.status.success(){
        eprintln!("Error: Failed to commit changes.");
        exit(1);
    }
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push changes");
    if !push_command.status.success(){
        eprintln!("Error: Failed to push changes to remote");
        exit(1);
    }
    println!("Successfully added, commited, and pushed all changes.")
}

fn name_generator() -> String {
    let mut input = String::new();
    println!("Write the commit message here: \n"  );
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input

    // let mut generator = Generator::default();
    // generator.next().unwrap()

}
