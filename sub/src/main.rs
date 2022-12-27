use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    sleep(Duration::from_secs(5));  // 休息5秒钟
    println!("Bye, world!");
}
