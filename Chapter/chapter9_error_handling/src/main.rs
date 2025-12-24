use std::backtrace;

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
    
}


fn main() {
    println!("Hello, world!");
}