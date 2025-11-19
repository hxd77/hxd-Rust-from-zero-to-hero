use std::io;
use std::cmp::Ordering;
use rand::Rng;//use rand::Rng; 是为了让 .gen_range() 这个 trait 方法在当前作用域中可见。rand::Rng相当于python中的import random
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);//rand::thread_rng()生成一个随机数生成器的实例返回一个Rng对象'

    //println!("The secret number is: {}",secret_number);//测试用，实际游戏中不需要打印这个数字
    
    loop{//loop表示无限循环，直到程序被手动终止或者遇到break语句
        println!("Please input your guess.");

        let mut guess =String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");//read_line函数会把用户输入的一行文本放到传入的字符串变量中，同时返回一个io::Result值，表示操作是否成功，如果io::Result值是Err，expect方法会导致程序崩溃并显示传入的错误信息
        
        let guess:u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=> continue,//_表示一个通配符，用来匹配所有Err值
        };//将输入的字符串转换为u32类型,trim()去掉字符串首尾的空白字符，parse()尝试将字符串解析为数字,:u32指定了要解析的数字类型

        println!("You geuessed: {}",guess);

        match guess.cmp(&secret_number){//一个分支，cmp方法返回一个Ordering枚举值，表示两个值的比较结果
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
            {println!("You win!");
            break; },
        }
    }
}
