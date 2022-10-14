use crate::parser::job;

use std::thread;
use std::process;

fn quit() {
    process::exit();
}

// Commands

fn exec(binary: &String, args: Vec<&String>) {
    // Command::new("cmd")
    //         .args(["/C", "echo hello"])
    //         .output()
    //         .expect("failed to execute process")

    // .output() = foreground
    // .spawn() = background
    // 
}

fn echo() {

}

fn export() {

}

fn cd() {

}

fn ls() {

}

fn kill() {

}

fn jobs() {

}

fn cat() {

}

/* Higher level abstractions */

fn pipe() {

}

fn redirect_io() {

}

fn background() {
    let handle = thread::spawn(|| {
        // todo
    });
}