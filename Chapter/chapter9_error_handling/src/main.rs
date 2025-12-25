use core::panic;
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

        match f.read_to_string(&mut s) //将文件f中的内容读出来放到s中,成功返回一个Ok(n),表示读取到的字节数
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
    
}
fn main() {
    println!("Hello, world!");
}