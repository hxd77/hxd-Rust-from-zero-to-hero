use std::io;
use rand::Rng;//use rand::Rng; 是为了让 .gen_range() 这个 trait 方法在当前作用域中可见。rand::Rng相当于python中的import random
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);//rand::thread_rng()生成一个随机数生成器的实例返回一个Rng对象'

    println!("The secret number is: {}",secret_number);//测试用，实际游戏中不需要打印这个数字

    println!("Please input your guess.");

    let mut guess =String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You geuessed: {}",guess);
}
