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

/**
闭包的写法
 */
pub fn closure() {
    fn create_counter() -> Box<dyn FnMut() -> i32> {
        let mut count = 0;
        Box::new(move || {
            count += 1;
            count
        })
    }
    let mut counter1 = create_counter();
    let mut counter2 = create_counter();
    for i in 0..3 {
        println!("counter1  {}", counter1());
    }
    for i in 0..1 {
        println!("counter2  {}", counter2());
    }
}

pub fn f_box() {
    fn print_string(s: Box<&str>) {
        println!("{}", s);
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn print(&self) {
            println!("({}, {})", self.x, self.y);
        }
    }
    let x = Box::new(5);
    println!("x = {}", x);

    // 创建一个装有字符串的 Box，并传递它的所有权给函数
    let s = Box::new("Hello, world!");
    print_string(s);

    // 创建一个装有自定义类型的 Box，并调用它的方法
    let p = Box::new(Point { x: 3, y: 4 });
    p.print();
}

