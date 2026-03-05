use core::panic;
use std::fmt::write;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use utils::*;
use colored::Colorize;

pub fn run(){
    print_section_title("第9章: 错误处理");

    //panic!与不可恢复的错误
    panic_examples();

    //Result与可恢复的错误
    result_examples();

    //错误传播
    error_propagation();

    //何时使用panic!
    when_to_panic();

    //自定义错误类型
    custom_error_types();
}

fn panic_examples(){
    print_example_title("9.1 panic!与不可回复的错误");

    //显示调用panic!
    explicit_panic();

    //由于bug产生的panic
    bug_panic();

    //使用panic的回溯
    backtrace_info();

    pause();
}   

fn explicit_panic(){
    println!("\n{}", "显式调用panic!：".blue().bold());

    //注意:这些代码在实际运行时会导致程序崩溃
    println!("程序即将panic!");

    //在实际代码中，下面的代码会导致程序崩溃
    //panic!("crash and burn")

    //我们用打印来模拟panic的行为
    println!("panic!('crash and burn')");
    println!("程序会在这里崩溃并显示错误信息");
}

fn bug_panic(){
    println!("\n{}", "由于bug产生的panic：".blue().bold());    

    let v=vec![1,2,3];

    //正常访问
    println!("v[0] = {}",v[0]);

    //下面的代码会导致panic
    //v[99]

    println!("访问v[99[会导致panic,因为索引超出范围");
    println!("这是一个缓冲区溢出的例子");
}

fn backtrace_info(){
    println!("\n{}", "使用panic!的回溯：".blue().bold());

    println!("要查看回溯信息，可以设置环境变量：");
    println!("RUST_BACKTRACE=1 cargo run");
    println!("或者设置为full获取更详细的信息：");
    println!("RUST_BACKTRACE=full cargo run");
}


fn result_examples(){
    print_example_title("9.2 Result与可恢复的错误");

    //处理Result
    handling_result();

    //匹配不同的错误
    matching_different_errors();

    //失败时panic的简写
    panic_shortcuts();

    pause();
}

fn handling_result(){
    println!("\n{}", "处理Result：".blue().bold());

    //尝试打开文件
    let filename="hello.txt";
    let result=file::open(filename);

    match result{
        Ok(file)=>println!("成功打开文件: {:?}",file),
        Err(error)=>println!("打开文件失败: {}",error),
    }

    //更简洁的错误处理
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("文件不存在，创建新文件");
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
}


fn matching_different_errors(){
    println!("\n{}", "匹配不同的错误：".blue().bold());

    let filename="hello.txt";
    let result=File::open(filename);

    let _file=match result{
        Ok(file)=>file,
        Err(error)=>match error.kind() {
            ErrorKind::NotFound=>{
                println!("文件不存在,尝试创建");
                match File::create(filename) {
                        Ok(fc)=>fc,
                        Err(e)=>panic!("创建文件失败: {:?}",e),
                }
            }
            other_error=>{
                panic!("打开文件遇到问题: {:?}",other_error);
            }
        }
    };

    println!("文件处理完成");
}

fn panic_shortcuts(){
    println!("\n{}", "失败时panic的简写：".blue().bold());

    //unwrap:如果Result是Ok,返回Ok中的值；如果是Err,调用panic!
    println!("使用unwrap(): ");
    let result=File::open("noneexistent.txt");
    match result{
        Ok(_)=>println!("文件打开成功"),
        Err(e)=>println!("unwrap会在这里panic: {}",e),
    }

    //expecdt:类似unwrap,但可以指定错误消息
    println!("\n使用expect():");
    let result=File::open("noneexistent.txt");
    match result{
        Ok(_) => println!("文件打开成功"),
        Err(e) => println!("expect会在这里panic with custom message: {}", e),
    }
}

fn error_propagation(){
    print_example_title("9.3 错误传播");
    
    // 传播错误
    propagating_errors();
    
    // ?运算符
    question_mark_operator();
    
    // ?运算符的链式调用
    chaining_question_mark();
    
    // ?运算符与main函数
    main_can_return_result();
    
    pause();
}

fn propagating_errors(){
   println!("\n{}", "传播错误：".blue().bold());

   fn read_user_name_from_file()->Result<String,io::Error>{
        let f=File::open("username.txt");

        let mut f=match f {
            Ok(file)=>file,
            Err(e)=>return Err(e),
        };

        let mut s=String::new();

        match f.read_to_string(&mut s) //将文件f中的内容读出来放到s中,成功返回一个Ok(string)
        {
            Ok(_)=>Ok(s),
            Err(e)=>Err(e),
        }
   }

   match read_user_name_from_file(){
        Ok(username)=>println!("用户名: {}",username),
        Err(e)=>println!("读取用户名失败: {}",e),
   }
}

fn question_mark_operator(){
    println!("\n{}", "?运算符：".blue().bold());

    fn read_username_from_file()->Result<String,io::Error>{
        let mut f=File::open("username.txt")?;
        let mut s=String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_username_from_file(){
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名失败: {}", e),
    }
    println!("?运算符使错误传播更简洁");
}

fn chaining_question_mark(){
    println!("\n{}", "?运算符的链式调用：".blue().bold());

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("username.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // 更简洁的版本
    fn read_username_from_file_short() -> Result<String, io::Error> {
        std::fs::read_to_string("username.txt")
    }

    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名失败: {}", e),
    }

    match read_username_from_file_short() {
        Ok(username) => println!("用户名（简短版本）: {}", username),
        Err(e) => println!("读取用户名失败（简短版本）: {}", e),
    }

}

fn main_can_return_result(){
    println!("\n{}", "?运算符与main函数：".blue().bold());

    println!("main函数可以返回Result<(), Box<dyn Error>>");
    println!("这允许在main函数中使用?运算符");

    //示例main函数
    println!("fn main() -> Result<(), Box<dyn Error>> {{");
    println!("    let f = File::open(\"hello.txt\")?;");
    println!("    Ok(())");
    println!("}}");
}

fn when_to_panic(){
    print_example_title("9.4 何时使用panic!");

    //示例、代码原型和测试
    examples_prototypes_tests();

    //当你比编译器知道更多信息时
    when_you_know_more();

    //错误处理指导原则
    error_handing_guidelines();

    pause();
}

fn examples_prototypes_tests(){
    println!("\n{}","示例、代码原型和测试: ".blue().bold());

    println!("在这些情况下使用unwrap和expect是合适的: ");
    println!("1. 编写示例代码时");
    println!("2. 快速原型开发时");
    println!("3. 编写测试时");

    //示例中的unwrap
    let result="42".parse::<i32>().unwrap(); //parse::<i32>比目鱼语法,把字符串转成32位有符号整数Result类型
    println!("解析结果: {}", result);

    //测试中的expect
    let result="42".parse::<i32>().expect("解析数字应该成功"); //expect成功返回Result里的值，失败则返回panic
    println!("解析结果: {}", result);
}

fn when_you_know_more(){
    println!("\n{}","当你比编译器知道更多信息时: ".blue().bold());

    use std::net::IpAddr;

    //我们知道这个字符串是有效的IP地址
    let home:IpAddr = "127.0.0.1".parse().unwrap();
    println!("家庭IP地址: {}", home);

    println!("在这种情况下，我们确信字符串是有效的IP地址");
    println!("所以使用unwrap是合适的");
}

fn error_handing_guidelines(){
    println!("\n{}", "错误处理指导原则：".blue().bold());

    println!("建议在以下情况下使用panic!：");
    println!("1. 代码可能会处于有害状态");
    println!("2. 有害状态是不被期望的");
    println!("3. 你的代码依赖于不处于这种有害状态");

    println!("\n建议返回Result的情况：");
    println!("1. 失败是预期的");
    println!("2. 调用者可以合理地处理错误");

    // 验证示例
    validation_example();
}

fn validation_example(){
    println!("\n{}","验证示例: ".blue().bold());

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("猜测值必须在1到100之间，得到了{}", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // 有效的猜测
    let guess = Guess::new(50);
    println!("有效的猜测: {}", guess.value());

    // 无效的猜测会导致panic
    // let invalid_guess = Guess::new(200);
    println!("创建无效猜测（值为200）会导致panic");
}

fn custom_error_types(){
    print_example_title("9.5 自定义错误类型");

    //定义自定义错误类型
    defining_custom_errors();

    //实现错误trait
    implementing_error_trait();

    //使用自定义错误类型
    using_custom_errors();

    pause();
}

fn defining_custom_errors(){
    println!("\n{}","定义自定义错误类型: ".blue().bold());

    #[derive(Debug)]
    enum MathError{
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    impl std::fmt::Display for MathError {
        //fmt是Display特征规定必须实现的方法
        //f是一个格式化器
        //write!是一个宏，表示把后面的中文字符串写入到前面的f中
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self{
                MathError::DivisionByZero => write!(f, "不能除以零"),
                MathError::NegativeLogarithm=>write!(f,"不能计算负数的对数"),
                MathError::NegativeSquareRoot=>write!(f,"不能计算负数的平方根"),
            }
        }

    }

    impl std::error::Error for MathError{} //实现MathError是一个标准的错误类型
    fn divide(a:f64,b:f64)->Result<f64,MathError>{
        if b==0.0{
            Err(MathError::DivisionByZero)
        }
        else { Ok(a/b) }
    }

    //使用自定义错误类型
    match divide(10.0,2.0){
        Ok(result)=>println!("10.0 / 2.0 = {}",result),
        Err(e)=>println!("错误: {}",e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
}

fn implementing_error_trait(){
    println!("\n{}", "实现错误trait：".blue().bold());

    #[derive(Debug)]
    enum ParsePersonErrorKind{
        Empty,
        BadLen,
        NoAge,
        ParseInt(std::num::ParseIntError), //()里面是返回错误类型
    }
    #[derive(Debug)]
    struct ParsePersonError{
        kind:ParsePersonErrorKind,
    }


}
fn main() {
    println!("Hello, world!");
}