#![allow(unused)]

use std::cmp::Ordering;
use std::io;
use rand::Rng;
use rust_future::model::user::User;
use rust_future::study::advanced::Advanced;
use rust_future::study::basic::ownership;

fn main() {
    println!("temp main!");
    // guess();
    // ownership();
    // Advanced::print_os_info()
    let zhang = User::new("zhang".to_string(), 19, "一个人".to_string());
    let mut lee = User {
        name: "lee".to_string(),
        age: 0,
        introduce: "lee introduce".to_string(),
    };
    lee.age = 14;
    zhang.print_self();
    lee.print_self()
}

fn guess() {
    println!("猜数游戏");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    let mut count = 0;
    loop {
        println!("输入你要猜测的数:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是:{}", guess);
        // 模式匹配
        // 调用cmp方法和secret_num 作比较，返回一个枚举值
        count = count + 1;
        match guess.cmp(&secret_num) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                print!("猜测次数: {}", count);
                break;
            }
        }
    }
}
