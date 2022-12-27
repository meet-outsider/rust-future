use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub struct Guess {}

impl Guess {
    pub fn guess() {
        println!("猜数游戏");
        let secret_num = rand::thread_rng().gen_range(1, 101);
        let mut count=0;
        loop {
            println!("输入你要猜测的数:");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("error");
            let guess:i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };
            println!("你猜测的数是:{}", guess);
            // 模式匹配
            // 调用cmp方法和secret_num 作比较，返回一个枚举值
            count=count+1;
            match guess.cmp(&secret_num) {
                Ordering::Less => {
                    println!("Too small!");
                }
                Ordering::Greater => {
                    println!("Too big!");
                }
                Ordering::Equal => {
                    println!("You win!");
                    print!("猜测次数: {}",count);
                    break;
                }
            }
        }
    }
}
