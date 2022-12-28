// mod guess_number;
// mod basic;

// use crate::basic::Basic;
// use crate::guess_number::Guess;
#![allow(unused)]

extern crate core;

use std::{env, thread};
use std::process::Command;
use std::thread::{sleep, Thread};
use std::time::Duration;
use crate::advanced::Advanced;
use crate::basic::{Basic, Coin};
use crate::guess_number::Guess;
use crate::sync_wait::SyncWait;

mod solution;
mod basic;
mod advanced;
mod sync_wait;
mod guess_number;

static mut S2: String = String::new();
static mut COUNT: i32 = 0;
static mut SYNC: SyncWait = SyncWait { count: 1 };

fn main() {
    ///
    /// 基础
    ///
    ///终端运行命令
    // Advanced::fn_command();
    ///猜数游戏
    // Guess::guess();
    ///基础变量
    // Basic::variable();
    ///复合类型
    // Basic::composite_type();
    ///循环
    // Basic::_loop();

    ///进阶
    ///
    /// 宏
    // Advanced::marco();
    ///输出OS信息
    // Advanced::print_os_info();


    Basic::match_fn(Coin::Dime)
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

unsafe fn thread_in() {
    sleep(Duration::from_secs(3));
    COUNT += 1;
    println!("diff count: {}", COUNT);
    SYNC.done();
}

fn change_ownership(num: i32, name: &mut String) {
    name.clear();
    match num % 2 == 0 {
        true => name.push_str("aaa"),
        false => name.push_str("ssss")
    }
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

