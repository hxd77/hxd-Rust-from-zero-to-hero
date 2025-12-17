use utils::*;
use std::collections::HashMap;
use colored::Colorize;

pub fn run()
{
    print_section_title("第8章: 常用集合类型");

    //Vector
    vector_examples();

    //字符串
    string_examples();

    //HashMap
    hash_map_exapmles();

    //综合练习
    comprehensive_exercises();
}

fn vector_examples()
{
    print_example_title("8.1 Vector");

    //创建Vector
    creating_vectors();

    //更新Vector
    updating_vectors();

    //读取Vector元素
    reading_vectors();

    //遍历Vector
    iteraing_vectors();

    //使用枚举存储多种类
    enum_in_vector();

    pause();
}
fn creating_vectors(){
    println!("\n{}","创建Vector: ".blue().bold());

    //创建空Vector
    let v:Vec<i32>=Vec::new();
    println!("空Vector: {:?}",v);

    //使用vec!泓创建包含初始值的Vector
    let v=vec![1,2,3];
    println!("使用vec!宏创建: {:?}",v);

    //创建带有初始容量的Vector
    let mut v:Vec<i32>=Vec::with_capacity(10);
    println!("带有初始容量的Vector: {:?}",v);
    println!("容量: {}",v.capacity());
}

fn updating_vectors(){
    println!("\n{}","更新Vector: ".blue().bold());

    let mut v=Vec::new();
    
    //添加元素
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("添加元素后: {:?}",v);

    // 弹出元素
    match v.pop() {
        Some(value) => println!("弹出的元素: {}", value),
        None => println!("Vector为空"),
    }

    println!("弹出后: {:?}",v);

    //插入元素
    v.insert(1, 42);
    println!("插入元素后: {:?}",v);

    //删除元素
    let removed=v.remove(1);
    println!("删除的元素: {}",removed);
    println!("删除后: {:?}",v);

    //清空Vector
    v.clear();
    println!("清空后: {:?}",v);
}

fn reading_vectors(){
    println!("\n{}","读取Vector元素: ".blue().bold());

    let v=vec![1,2,3,4,5];
    
    //使用索引
    let third=&v[2];
    println!("第三个元素: {}",third);

    //使用get方法
    let third=v.get(2);
    match v.get(2){
        Some(third)=>println!("第三个元素: {}",third),
        None=>println!("没有第三个元素"),
    }

    //超出范围的索引
    let does_not_exist=v.get(100);
    println!("不存在的索引: {:?}",does_not_exist);

    //注意: 下面的代码会导致panic
    //let does_not_exist=&v[100];
    
    //不可变引用
    let mut v=vec![1,2,3,4,5];
    let first=&v[0]; //不可变引用
    //v.push(6);//这会导致编译错误
    println!("第一个元素: {}",first);

}

fn iteraing_vectors()
{
    println!("\n{}","遍历Vector: ".blue().bold());

    let v=vec![100,32,57];

    //遍历不可变引用
    for i in &v{
        println!(" {}",i);
    }

    //遍历可变引用
    let mut v=vec![100,32,57];
    println!("遍历可变引用");
    for i in &mut v{
        *i+=50;
        println!(" {}",i);
    }

    //遍历拥有所有权
    println!("遍历拥有所有权: ");
    for i in v{
        println!(" {}",i);
    }
    //注意: v在这里已经不能用了
}

fn enum_in_vector(){
    println!("\n{}","使用枚举存储多种类型: ".blue().bold());

    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row=vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("电子表格行: {:?}",row);

    //处理不同类型的值
    for cell in &row{
        match cell{
            SpreadsheetCell::Int(i)=>println!("整数: {}",i),
            SpreadsheetCell::Float(f)=>println!("浮点数: {}",f),
            SpreadsheetCell::Text(s)=>println!("文本: {}",s),
        }
    }
}

fn string_examples(){
    print_example_title("8.2 字符串");

    //创建字符串
    creating_strings();

    //更新字符串
    updating_strings();

    //索引字符串
    indexing_strings();

    //切片字符串
    slicing_strings();

    //遍历字符串
    iterating_strings();

    pause();
}

fn creating_strings(){
    println!("\n{}","创建字符串: ".blue().bold());

    //创建空字符串
    let mut s=String::new();
    println!("空字符串: '{}'",s);

    //从字符串字面量创建
    let data="initial contents";
    println!("从字符串字面量创建: '{}'",s);

    //使用String::from
    let s=String::from("inital contents");
    println!("使用String::from: '{}'",s);

    //包含UTF-8字符的字符串
    let hello=String::from("السلام عليكم");
    println!("阿拉伯语: {}",hello);

    let hello=String::from("Dobrý den");
    println!("捷克语: {}",hello);

     let hello = String::from("Hello");
    println!("英语: {}", hello);
    
    let hello = String::from("שָׁלוֹם");
    println!("希伯来语: {}", hello);
    
    let hello = String::from("नमस्ते");
    println!("印地语: {}", hello);
    
    let hello = String::from("こんにちは");
    println!("日语: {}", hello);
    
    let hello = String::from("안녕하세요");
    println!("韩语: {}", hello);
    
    let hello = String::from("你好");
    println!("中文: {}", hello);
    
    let hello = String::from("Olá");
    println!("葡萄牙语: {}", hello);
    
    let hello = String::from("Здравствуйте");
    println!("俄语: {}", hello);
    
    let hello = String::from("Hola");
    println!("西班牙语: {}", hello);

}

fn updating_strings(){
    println!("\n{}","更新字符串: ".blue().bold());

    //使用push_str追加字符串切片
    let mut s=String::from("foo");
    s.push_str("bar");
    println!("使用push_str: {}",s);

    //push_str不会获取参数的所有权
    let mut s1=String::from("foo");
    let s2="bar";
    s1.push_str(s2);
    println!("s1: {},s2: {}",s1,s2);

    //使用push追加字符
    let mut s=String::from("lo");
    s.push('l');
    println!("使用push: {}",s);

    //使用+连接字符串
    let s1=String::from("Hello, ");
    let s2=String::from("world!");
    let s3=s1+&s2; //注意s1被移动了,不能继续使用
    println!("使用+连接: {}",s3);

    //多个字符串连接
    let s1=String::from("tic");
    let s2=String::from("tac");

    
    //使用format!宏
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    let s=format!("{}-{}-{}",s1,s2,s3);
    println!("使用format!宏: {}",s);
    println!("s1, s2 ,s3仍然可用: {},{},{}",s1,s2,s3);
}

fn indexing_strings(){
    println!("\n{}","索引字符串: ".blue().bold());

    let s1=String::from("hello");
    //let h =s1[0]; //这会导致编译错误

    println!("字符串索引在Rust中是不被允许的");

    //字符串的内部表示
    let hello=String::from("Hola");
    println!("Hola 的长度: {}",hello.len());

    let hello = String::from("Здравствуйте");
    println!("'Здравствуйте'的长度: {}", hello.len());
    
    // 字节、标量值和字形簇
    let hello = "नमस्ते";
    println!("'नमस्ते'的不同视图:");
    println!("  字节: {:?}", hello.as_bytes());
    println!("  字符: {:?}", hello.chars().collect::<Vec<char>>());
    // 字形簇需要外部crate
}

fn slicing_strings(){
    println!("\n{}","切片字符串: ".blue().bold());

    let hello="Здравствуйте";
    
}
fn main()
{
    run();
}