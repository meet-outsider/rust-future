// mod guess_number;
// mod basic;

// use crate::basic::Basic;
// use crate::guess_number::Guess;
#![allow(unused)]

use std::env;
use crate::advanced::Advanced;

mod solution;
mod basic;
mod advanced;

static mut S2: String = String::new();

fn main() {

    // let solution=Solution::new();
    // let solution = ::contains_duplicate( vec![1, 2, 3,1]);
    // println!("{}",solution.contains_duplicate(vec![1, 2, 3,1]))
    // let output = Command::new("ls").output().expect("执行异常，提示");
    // let out = String::from_utf8(output.stdout).unwrap();
    // println!("{}", out);
    // Guess::guess();
    // Basic::variable();
    // Basic::composite_type();
    // Basic::_loop();


    // let mut mut_str = String::from("mutStr");
    // unsafe { S2 = String::from("s2 init"); }
    // unsafe { todo(&mut mut_str); }

    // // Advanced::marco();
    // if cfg!(target_os = "windows") {
    //     println!("Hello Windows");
    // } else if cfg!(target_os = "linux") {
    //     println!("Hello Linux");
    // } else if cfg!(target_os = "mac") {
    //     println!("Hello mac");
    // } else {
    //     println!("Unknown os");
    // }
    // println!("{}", env::consts::OS); // Prints the current OS.}


    /// key：result value:ccc
    let result = String::from("ccc"); ///'ccc'的所有权输入result
    // result = "aa";
    let (status, msg) = change(&result); ///借用'ccc'的所有权，目前还是属于result
    let a=&result; ///所有权移动到了a
    println!("result :{}", result);
    println!("a :{}", a);
    println!("{} {}", status, msg)
}

fn change(name: &String) -> (bool, String) {
    println!("{}", name);
        // name.clear();
    (true, String::from("aaa"))
}

unsafe fn todo(mut_str: &mut String) {
    let s1 = String::from("hello");
    // S2 = s1;
    println!("s1 value is :{}", s1);
    println!("s2 value is :{}", S2);
    println!("mut_str value is :{}", mut_str);
}


