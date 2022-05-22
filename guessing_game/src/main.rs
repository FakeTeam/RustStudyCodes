use std::io;

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let condition = true;
    // 条件初始化
    let number = if condition { 5 } else { 6 };
    println!("Init number  = {}", number);
    let srect_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", srect_number);
    loop {
        println!("Please input your guess number.");
        //let 类似 goalng 中的var c++ 中的 auto 自动推导
        // rust 中所有变量都是不可修改的 需要添加mut来修改 类似c++中的 const 添加 mutable
        let mut guess = String::new();
        //expect 相当于捕获异常 实际上就是 Error
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("error!!!! {} {}", guess, &e);
                continue;
            }
        };
        println!("You guessed :{} srect_number:{}", guess, srect_number);
        match guess.cmp(&srect_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
