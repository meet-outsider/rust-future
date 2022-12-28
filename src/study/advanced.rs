#![allow(unused)]

use std::env;
use std::process::Command;

pub struct Advanced {}

macro_rules! yo {
    ($name:expr) => {
        println!("Yo {}!",$name)
    };
}
impl Advanced {
    pub fn marco() {
        yo!("Fini")
    }
    pub fn fn_command() {
        let output = Command::new("ls").output().expect("执行异常，提示");
        let out = String::from_utf8(output.stdout).unwrap();
        println!("{}", out);
    }
    pub fn print_os_info() {
        if cfg!(target_os = "windows") {
            println!("Hello Windows");
        } else if cfg!(target_os = "linux") {
            println!("Hello Linux");
        } else if cfg!(target_os = "macos") {
            println!("Hello mac");
        } else {
            println!("Unknown os");
        }
        println!("{}", env::consts::OS); // Prints the current OS.}
    }
}
