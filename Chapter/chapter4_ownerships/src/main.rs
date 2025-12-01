use utils::*;//引入crate（项目）中utils模块里的所有公开项 
use colored::Colorize;//引入colored库的Colorize trait，用于给终端输出添加颜色

pub fn run()//pub表示这个函数是公共的，可以被其他模块调用
{
    print_section_title("第四章：所有权系统");

    //所有权
    ownership_basics();

    //引用和借用
    references_and_borrowing();

    //切片类型
    slice_type();

}

fn ownership_basics(){
    print_example_title("4.1 所有权基础");

    //所有权规则
    ownership_rules();

    //变量作用域
    variable_scope();

    //内存分配
    memory_allocation();

    //移动语义
    move_semantics();

    //克隆
    clone_example();

    //栈上的数据：复制
    stack_only_data();

    //所有权与函数
    ownership_and_functions();

    //返回值与作用域
    return_values_and_scope();

    pause();

}

fn ownership_rules()
{
    println!("\n{}","所有权规则：".blue().bold());
    println!("1. Rust中的每一个值都有一个被称为其所有者(owner)的变量");
    println!("2. 值在任一时刻有且只有一个所有者");
    println!("3. 当所有者（变量）离开作用域，这个值将被丢弃");
}

fn variable_scope(){
    println!("\n{}","变量作用域：".blue().bold());
    {           //s在这里无效，它尚未声明
        let s="hello";  //从此处起，s是有效的
        println!("s的值:{}",s); //使用s
    }           //此作用域结束，s不再有效

    //println!("s的值:{}",s);//这会导致编译错误

    println!("变量s已经离开作用域");
}

fn memory_allocation(){
    println!("\n{}","内存分配".blue().bold());

    //字符串字面量是不可变的
    let s1="hello";
    println!("字符串字面量：{}",s1);

    //String类型是可变的，在堆上分配
    let mut s2=String::from("hello");
    s2.push_str(", world");//push_str()在字符串后追加字面值
    println!("String类型：{}",s2);

    //当s2离开作用域时，drop函数会被调用
}

fn move_semantics(){
    println!("\n{}","移动语义：".blue().bold());
    let s1=String::from("hello");
    let s2=s1;  //s1被移动到s2

    //println!("s1：{}",s1);    //这会导致编译错误，因为s1已经被移动
    println!("s2：{}",s2);    
    
    //对于栈上的数据，是复制而不是移动
    let x=5;
    let y=x;  //x被复制到y

    println!("x:{},y:{}",x,y);  //这是有效的

}

fn clone_example()
{
    println!("\n{}","克隆:  ".blue().bold());
    
    let s1=String::from("hello");
    let s2=s1.clone(); //深拷贝

    println!("s1:{},s2:{}",s1,s2); //两个都有效
}

fn stack_only_data()
{
    println!("\n{}","栈上的数据: ".blue().bold());
    let x=5;
    let y=x;  //x被复制到y

    println!("x:{},y:{}",x,y);  //这是有效的

    //实现了Copy trait的类型,一个旧的变量在将其赋值给其他变量后仍然可用。
    //整数类型、布尔类型、浮点类型、字符类型、元组（只包含实现了Copy trait的类型）
    let tuple=(5,10,"a");
    let tuple_copy=tuple;
    println!("原tuple:{:?},复制的tuple_copy:{:?}",tuple,tuple_copy);
}

fn ownership_and_functions()
{
    println!("\n{}","所有权与函数: ".blue().bold());

    let s=String::from("hello");//s进入作用域

    takes_ownership(s);//s的值被移动到函数里
    //到这里，s不再有效
    //println!("{}",s); //这会导致编译错误

    let x=5;//x进入作用域
    make_copy(x);   //x应该移动函数里,但i32是Copy的，所以在后面可继续使用x
    println!("{}",x); //这是有效的
}//这里，x先移出了作用域，然后是s。但因为s的值已被移走

fn takes_ownership(some_string:String)   //some_string进入作用域
{
    println!("{}",some_string);
}//这里，some_string离开作用域并调用drop函数，释放内存

fn make_copy(some_integer:i32)//some_integer进入作用域
{
    println!("{}",some_integer);
}//这里，some_integer离开作用域。没有特殊操作

fn return_values_and_scope()
{
    println!("\n{}","返回值与作用域: ".blue().bold());

    let s1=gives_ownership(); //gives_ownership将返回值移给s1
    println!("s1: {}",s1);

    let s2=String::from("hello");   //s2进入作用域
    
    let s3=takes_and_gives_back(s2);//s2并移动到takes_and_gives_back中，它也将返回值移给s3
    println!("s3: {}",s3);
    //println!("s2: {}",s2); //这会导致编译错误
}//这里，s3移出作用域并被丢弃。s2也移出作用域，但已被移走，
//所以这里什么也不会发生，s1移出作用域并被丢弃

fn gives_ownership()->String    //gives_ownership将返回值移动给调用它的函数
{
    let some_string =String::from("yours"); //some_string进入作用域
    some_string //返回some_strin并移出给调用的函数
}

//takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string:String)->String
{
    //a_string进入作用域
    a_string //返回a_string并移出给调用的函数
}

fn references_and_borrowing()
{
    print_example_title("4.2 引用与借用");
    
    //引用
    reference_example();

    //可变引用
    mutable_references();

    //引用的规则
    reference_rules();

    //悬垂引用
    dangling_reference();

    pause();
}

fn reference_example()
{
    println!("\n{}","引用: ".blue().bold());
    
    let s1=String::from("hello");
    let len=calculate_length(&s1);

    println!("字符串'{}'的长度是{}",s1,len);
}

fn calculate_length(s:&String)->usize//以一个对象的引用作为参数而不是获取值的所有权,引用允许你使用值但不获取其所有权,所以当引用停止使用时，它所指向的值也不会被丢弃
{
   s.len()
}//这里,s离开了作用域。但因为它并不拥有引用值得所有权，所以什么也不会发生


fn mutable_references()
{
    println!("\n{}","可变引用: ".blue().bold());

    let mut s=String::from("hello");
    change(&mut s);
    println!("修改后的字符串: {}",s);

    //可变引用的限制
    let mut s=String::from("hello");
    {//使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能同时拥有
        let r1=&mut s;
        println!("r1: {}",r1);
    }//r1在这里离开了作用域，所以我们完全可以创建一个新的引用
    
    let r2=&mut s;
    println!("r2: {}",r2);
}

fn change(some_string:&mut String)
{
    some_string.push_str(", world");
}

fn reference_rules()
{
    println!("\n{}","引用的规则".blue().bold());
    println!("1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用");
    println!("2. 引用必须总是有效的");

    let mut s=String::from("hello");

    let r1=&s;//没问题
    let r2=&s;//没问题
    //不能在拥有不可变引用的同时拥有可变引用，注意一个引用的作用域从声明得地方开始一直持续到最后一次使用为止
    println!("{} and {}",r1,r2);
    
    let r3=&mut s;//没问题
    println!("{}",r3);
}

fn dangling_reference()
{
    println!("\n{}","悬垂引用: ".blue().bold());
    println!("Rust编译器会阻止悬垂引用得产生");
    
    //下面的代码会导致编译错误
    //let reference_to_nothing=dangele();

    //正确的方式
    let valid_string=no_dangle();
    println!("有效的字符串: {}",valid_string);
}

//这个函数会导致编译错误
/*
fn dangle()->&String//dangle()返回一个字符串的引用
{
    let s=String::from("hello");//s是一个新字符串
    &s//返回字符串s的引用
} */ //这里s离开作用域并被丢弃。其内存被释放,危险!

fn no_dangle()->String
{
    let s=String::from("hello");
    s//直接返回String
}

fn slice_type()
{
    print_example_title("4.3 切片类型");

    //字符串切片
    string_slices();

    //其他切片
    other_slices();

    pause();
}

fn string_slices()//string slice是String中一部分值的引用
{
    println!("\n{}","字符串切片".blue().bold());

    let s=String::from("hello world");

    let hello=&s[0..5];
    let world=&s[6..11];

    println!("hello: {}",hello);
    println!("world: {}",world);

    //切片语法
    let s=String::from("hello");
    
    let slice =&s[0..2];
    println!("slice: {}",slice);

    let slice=&s[..2];//等价于&s[0..2]
    println!("slice: {}",slice);

    let slice=&s[3..];//等价于&s[3..len]
    println!("slice: {}",slice);

    let slice=&s[..];//等价于&s[0..len]
    println!("slice: {}",slice);

    //实际应用：获取第一个单词
    let s=String::from("hello world");
    let word=first_word(&s);
    println!("第一个单词: {}",word);

    //s.clear();//这会导致编译错误，因为word是s的不可变引用

    //字符串字面量就是切片str
    let s="Hello,world!";
    println!("字符串字面量: {}",s);

    //改进的函数
    let my_string=String::from("hello world");
    
    //first_word_improve 接受 String的切片，无论是部分还是全部
    let word=first_word_improve(&my_string[0..6]);
    println!("改进函数的结果: {}",word);

    let word=first_word_improve(&my_string[..]);
    println!("改进函数的结果: {}",word);

    //first_word_improve也接受String的引用
    //这等同于String的全部切片
    let word =first_word_improve(&my_string);
    println!("改进函数的结果: {}",word);

    let my_string_literal="hello world";
    println!("改进函数的结果: {}",word);

    //first_word_improve接受字符串字面量的切片，无论是部分还是全部
    let word=first_word_improve(&my_string_literal[0..6]);
    println!("改进函数的结果: {}",word);

    let word=first_word_improve(&my_string_literal[..]);
    println!("改进函数的结果: {}",word);

    //因为字符串字面值就是字符串slice
    //这样写也可以，即不适用slice语法
    let word=first_word_improve(&my_string_literal);
    println!("改进函数的结果: {}",word);
    

}

fn first_word(s:&String)->&str
{
    let bytes=s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_improve(s:&str)->&str
{
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn other_slices()
{
    println!("\n{}","其他切片: ".blue().bold());

    let a=[1,2,3,4,5];
    let slice=&a[1..3];//等价于[2,3]

    println!("数组: {:?}",a);
    println!("切片: {:?}",slice);

    assert_eq!(slice,&[2,3]);
    println!("断言成功: 切片等于[2,3]");
}   
fn main() {
    run();
}
