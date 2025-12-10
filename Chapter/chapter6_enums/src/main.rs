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

fn pattern_matching(){
    print_example_title("6.2 模式匹配");

    //match表达式
    match_expression();

    //绑定值的模式
    binding_values();

    //匹配Option<T>
    matching_option();

    //通配符模式
    wildcard_patterns();

    pause();
}

fn match_expression()
{
    println!("\n{}","match表达式".blue().bold());

    #[derive(Debug)]
    enum Coin
    {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin:Coin)->u8{
        match coin{
            Coin::Penny=>{
                println!("幸运硬币");
                1
            }
            Coin::Nickel=>2,
            Coin::Dime=>10,
            Coin::Quarter=>25,
        }
    }

    let coin=Coin::Penny;
    let value=value_in_cents(coin);
    println!("硬币价值: {}分",value);
}

fn binding_values()
{
    println!("\n{}","绑定值的模式: ".blue().bold());

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        //..等等
    }

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),//只有25美分硬币有特殊价值
    }

    fn value_in_cents(coin:Coin)->u8
    {
        match coin{
            Coin::Penny=>1,
            Coin::Nickel=>5,
            Coin::Dime=>10,
            Coin::Quarter(state)=>{
                println!("来自{:?}州的25分硬币",state);
                25
            }
        }
    }
    let coin=Coin::Quarter(UsState::Alaska);
    let value=value_in_cents(coin);
    println!("硬币价值: {} 分",value);
}

fn matching_option()
{
    println!("\n{}","匹配Option<T>: ".blue().bold());

    fn plus_one(x:Option<i32>)->Option<i32>{
        match x{
            None=>None,
            Some(i)=>Some(i+1),
        }
    }
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);
}

fn wildcard_patterns()
{
    println!("\n{}","通配符模式: ".blue().bold());

    let dice_roll=9;
    match dice_roll{
        3=>add_fancy_hat(),
        7=>remove_fancy_hat(),
        other=>move_player(other),
    }

    //使用 _ 通配符
    let dice_roll=9;

    match dice_roll{
        3=>add_fancy_hat(),
        7=>remove_fancy_hat(),
        _ =>reroll(),
    }

    //什么都不做
    let dice_roll=9;

    match dice_roll {
        3=>add_fancy_hat(),
        7=>remove_fancy_hat(),
        _=>(),
    }
    fn add_fancy_hat()
    {
        println!("添加花哨的帽子");
    }

    fn remove_fancy_hat()
    {
        println!("移除花哨的帽子");
    }

    fn move_player(num_spaces:u8)
    {
        println!("移动玩家{}步",num_spaces);
    }

    fn reroll()
    {
        println!("重新掷骰子");
    }
}


fn if_let_syntax()
{
    print_example_title("6.3 if let 语法");

    //if let 简化匹配
    let config_max=Some(3u8);

    //使用match
    match config_max
    {
        Some(max)=>println!("最大值是{}",max),
        _=>(),
    }

    //使用if let 
    if let Some(max)=config_max{
        println!("最大值是{}",max);
    }

    println!("if let 语法更简洁");
}

fn if_let_with_else()
{
    println!("\n{}","if let 与else: ".blue().bold());

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin=Coin::Penny;
    let mut count=0;
    
    if let Coin::Quarter(state)=coin{
        println!("来自{:?}州的25分硬币",state);
    }
    else {
        count+=1;
    }
    println!("非25分硬币技术: {}",count);
}
fn main()
{
    run();
}