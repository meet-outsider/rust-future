pub struct Basic {}

impl Basic {
    pub fn _variable() {
        // 变量不可变
        let var = 3;
        // wrong case
        // var =4;
        println!("the value of var is {}", var);

        // let mut var = var + 1;
        // var = 3;
        // var=4;
        // 变量遮蔽 shadowing
        let x = 3;
        let x = x + 3;
        println!("the value of var is {}", var + 1);
        println!("the value of x is {}", x);
        let guess: i32 = "45".parse().expect("not a number!");
        println!("guess: {}", guess);
    }
    // pub fn _composite_type() {
    //     // 初始化数组
    //     let array1 = [2, 3, 2, 4, 1];
    //     // 初始化值为0，长度为4
    //     let array2 = [0; 4];
    //     for x in array1 {
    //         print!("{}\t", x)
    //     }
    //     println!();
    //     for x in array2 {
    //         print!("{}\t", x)
    //     }
    //     let _ = {
    //         let _x = 3;
    //         _x + 1;
    //     };
    //     // println!("the value of y is {}", y);
    // }

    // pub fn _loop() {
    //     let mut  count = 0;
    //
    //     let number = loop {
    //         count += 1;
    //         if count == 10 {
    //             break count * 2;
    //         }
    //     };
    //     println!("the value of number is {}",number)
    // }

    pub fn _move_copy(){
        println!("基础类型（在栈上的操作）都是copy的，性能高");
        print!("复杂类型（在堆上操作）几乎都是移动操作，性能高，如果需要深拷贝，需要使用clone方法）");
    }
}
