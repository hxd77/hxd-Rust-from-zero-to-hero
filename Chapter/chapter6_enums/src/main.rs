use std::str::pattern::Pattern;

use utils::*;
use colored::Colorize;

pub fn run()
{
    print_section_title("第6章: 枚举");

    //枚举
    enums_basics();

    //模式匹配
    pattern_matching();

    //if let语法
    if_let_syntax();
}

fn enums_basics()
{
    print_example_title("6.1 枚举基础");

    //定义枚举
    enum_definition();

    //将数据附加到枚举成员
    enum_with_data();

    //Option枚举
    optino_enum();

    pause();
}

#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

fn route(ip_kind:IpAddrKind){
    println!("路由IP类型:{:?}",ip_kind);
}

fn enum_definition(){
    println!("\n{}","定义枚举: ".blue().bold());

    let four=IpAddrKind::V4;
    let six=IpAddrKind::V6;

    println!("IPv4: {:?}",four);
    println!("IPv6: {:?}",six);
    
    //枚举作为函数参数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn enum_values()
{
    println!("\n{}","枚举值: ".blue().bold());

    #[derive(Debug)]
    enum IpAddrKind{
        V4,
        V6,
    }

    struct IpAddr{
        kind:IpAddrKind,
        address:String,
    }

    let home=IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };

    let loopback=IpAddr{
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };
    
    println!("家庭IP: {:?} {}",home.kind,home.address);
    println!("环回IP: {:?} {}",loopback.kind,loopback.address);
}

#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self)
    {
        println!("调用消息: {:?}",self);
    }
}

fn enum_with_data()
{
    println!("\n{}","将数据附加到枚举成员: ".blue().bold());

    #[derive(Debug)]
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home =IpAddr::V4(127,0,0,1);
    let loopback=IpAddr::V6(String::from("::1"));

    println!("家庭IP: {:?}",home);
    println!("环回IP: {:?}",loopback);

    //更复杂的例子
    let msg1=Message::Quit;
    let msg2=Message::Move { x:10, y: 20 };
    let msg3=Message::Write(String::from("hello"));
    let msg4=Message::ChangeColor(255, 0, 0);

    println!("消息1: {:?}",msg1);
    println!("消息2: {:?}",msg2);
    println!("消息3: {:?}",msg3);
    println!("消息4: {:?}",msg4);
}

fn optino_enum()
{
    println!("\n{}","Option枚举: ".blue().bold());

    let some_number=Some(5);
    let some_string=Some("a string");

    let absent_number:Option<i32>=None;

    println!("某个数字: {:?}",some_number);
    println!("某个字符串: {:?}",some_string);
    println!("缺失的数字: {:?}",absent_number);
     
    //Option的使用场景
    let x:i8=5;
    let y:Option<i8>=Some(5);

    //let sum=x+y; //这会导致编译错误因为类型不同一个是i8,一个是Option<i8>
    //必须将Option<T>转换为T才能使用
}

fn main()
{
    run();
}