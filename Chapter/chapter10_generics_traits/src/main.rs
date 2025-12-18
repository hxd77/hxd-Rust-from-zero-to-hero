use utils::*;
use colored::Colorize;
use std::collections::btree_map::Values;
use std::fmt::{format, write};
use std::ops::Add;
use std::{default, fmt};

pub fn run()
{
    print_section_title("第10章: 泛型和Trait");

    //泛型数据类型
    generic_data_types();

    //Trait: 定义共同行为
    traits_defining_shared_behavior();

    //默认实现
    default_implementations();

    //Traits作为参数
    traits_as_parameters();

    //Trait Bound
    trait_bounds();

    //返回实现了Trait的类型
    returing_trait_types();

    //有条件地实现方法
    conditional_implementations();

    //孤儿规则
    orphan_rule();

    //运算符重载
    operator_overloading();

    //高级trait特性
    advanced_traits();
}

fn generic_data_types(){
    print_example_title("10.1 泛型数据类型");

    //泛型函数
    generic_functions();

    //泛型结构体
    generic_structs();

    //泛型枚举
    generics_enums();

    //泛型方法
    generic_methods();

    //泛型的性能
    generic_performance();

    pause();
}
fn generic_functions(){
    println!("\n{}","泛型函数: ".blue().bold());

    //查找最大值得泛型函数
    fn largest<T:PartialOrd+Copy>(list:&[T])->T{
        let mut largest=list[0];
        for &item in list{
            if item>largest{
                largest=item;
            }
        }
        largest
    }
    /*
    2️⃣ T: PartialOrd + Copy（核心）
    这是 trait 约束（trait bounds），意思是：类型 T 必须同时实现：
    PartialOrd（可比较大小）
    Copy（可以按位复制）
    🔹 PartialOrd
    允许使用 >、<、>= 等比较运算
    没有这个约束，item > largest 会编译失败
    🔹 Copy
    允许 T 在赋值、返回时 按值复制
    没有 Copy，下面代码会报错：
     */
    let number_list=vec![34,50,25,100,65];
    let result=largest(&number_list);
    println!("最大的数字是{}",result);

    let char_list=vec!['y','m','a','q'];
    let result=largest(&char_list);
    println!("最大的字符是{}",result);

    //多个泛型参数
    fn compare<T,U>(a:T,b:U)->String
    where 
        T:std::fmt::Display,
        U:std::fmt::Display,
        /*要求泛型类型 T 和 U 都必须实现 Display trait，跟#[derived(Debug)]不一样
        也就是：它们都能用 {} 的方式格式化输出。 */
    {
        format!("a: {},b: {}",a,b)
    }
    
    let result=compare(10, "hello");
    println!("比较结果: {}",result);
}

fn generic_structs(){
    println!("\n{}","泛型结构体: ".blue().bold());

    //单个泛型参数
    #[derive(Debug)]
    struct Point<T>{
        x:T,
        y:T,
    }

    let integer=Point{x:5,y:10};
    let float=Point{x:1.0,y:4.0};

    println!("整数点: {:?}",integer);
    println!("浮点数点: {:?}",float);

    //多个泛型参数
    #[derive(Debug)]
    struct MixedPoint<T,U>{
        x:T,
        y:U,
    }

    let mixed=MixedPoint{x:5,y:4.0};
    println!("混合点: {:?}",mixed);

    //带约束的泛型结构体
    #[derive(Debug)]
    struct Pair<T>{
        first:T,
        second:T,
    }

    impl<T> Pair<T>{
        fn new(first:T,second:T)->Self{
            Self{first,second} //结构体初始化的简写语法
        }
    }

    impl<T:PartialOrd> Pair<T>{ //泛型加PartialOrd trait,要求类型T必须实现这个trait
        fn cmp_display(&self)->&T{
            if self.first>=self.second{
                &self.first
            }
            else {
                &self.second
            }
        }
    }
    
    let pair=Pair::new(3, 7);
    println!("较大的值: {}",pair.cmp_display());
}

fn generics_enums(){
     println!("\n{}", "泛型枚举：".blue().bold());

     //Option和Result都是泛型枚举
     let some_number=Some(5);
     let some_string=Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);

    //自定义泛型枚举
    #[derive(Debug)]
    enum MyResult<T,E>{
        Success(T),
        Failure(E),
    }

    let success:MyResult<i32,String>=MyResult::Success(42);
    let failure:MyResult<i32, String>=MyResult::Failure("错误".to_string());

    println!("成功: {:?}",success);
    println!("失败: {:?}",failure);

    //处理泛型枚举
    match success {
        MyResult::Success(value)=>println!("成功值: {}",value),
        MyResult::Failure(error)=>println!("错误: {}",error),
    }
}

fn generic_methods(){
     println!("\n{}", "泛型方法：".blue().bold());

     struct Point<T>{
        x:T,
        y:T,
     }

     impl<T> Point<T>{
        fn x(&self)->&T{
            &self.x
        }

        fn y(&self)->&T{
            &self.y
        }
     }

     //只为特定类型(f32)实现方法
     impl Point<f32>{
        fn distance_from_origin(&self)->f32{
            (self.x.powi(2)+self.y.powi(2)).sqrt()
        }
     }

     let point=Point{x:5,y:10};
     println!("x = {}",point.x());
     println!("y = {}", point.y());

     let float_point=Point{x:3.0,y:4.0};
     println!("距离原点: {}", float_point.distance_from_origin());

     //混合泛型方法
     struct MixedPoint<T,U>{
        x:T,
        y:U,
     }
     impl <T,U> MixedPoint<T,U>{
        fn mixup<V,W>(self,other:MixedPoint<V,W>)->MixedPoint<T,W>{
            MixedPoint { 
                x:self.x, 
                y:other.y,
            }
        }
     }

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("混合后: x = {}, y = {}", p3.x, p3.y);
}

fn generic_performance(){
    println!("\n{}", "泛型的性能：".blue().bold());
    
    println!("Rust的泛型在运行时没有性能损失");
    println!("编译器会进行单态化(monomorphization)");
    println!("每个具体类型都会生成特定的代码");
    
    // 示例：编译器会为每个具体类型生成专门的代码
    let integer = Some(5);
    let float = Some(5.0);
    
    // 编译后实际上是两个不同的类型
    println!("Integer Option: {:?}", integer);
    println!("Float Option: {:?}", float);
}

fn traits_defining_shared_behavior(){
    print_example_title("10.2 Trait: 定义共同行为");

    //定义trait
    defining_traits();

    //为类型实现trait
    implementing_traits();

    //trait与泛型结合
    traits_with_generics();

    pause();
}

fn defining_traits() {
    println!("\n{}", "定义trait：".blue().bold());
    
    // 定义一个简单的trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    
    // 定义包含默认实现的trait
    pub trait Display {
        fn format(&self) -> String {
            String::from("(读取更多...)")
        }
    }
    
    // 定义新闻文章结构体
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    impl Display for NewsArticle {}
    
    // 定义推特结构体
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    impl Display for Tweet {
        fn format(&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
    }
    
    // 使用trait
    let article = NewsArticle {
        headline: "Rust 1.70发布".to_string(),
        location: "全球".to_string(),
        author: "Rust Team".to_string(),
        content: "Rust 1.70带来了许多新特性...".to_string(),
    };
    
    let tweet = Tweet {
        username: "rust_lang".to_string(),
        content: "Rust编程语言官方推特".to_string(),
        reply: false,
        retweet: false,
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("文章格式: {}", article.format());
    println!("推特摘要: {}", tweet.summarize());
    println!("推特格式: {}", tweet.format());
}

fn implementing_traits(){
    println!("\n{}", "为类型实现trait：".blue().bold());

    //为自定义类型实现标准库trait
    #[derive(Debug)]
    struct Rectangle{
        width :u32,
        height:u32,
    }

    //实现Dispaly trait
    impl std::fmt::Display for Rectangle{
        fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result { //f:&mut fmt::Formatter表示一个输出缓冲区
        write!(f, "Rectangle{{ width: {}, height: {}}}",self.width,self.height) //把内容写入f      
        }
    }

    //实现PartialEq trait
    impl PartialEq for Rectangle{
        fn eq(&self,other:&Self)->bool{ //self表示方法的调用者,Self表示Rectangle一种类型
            self.width==other.width&&self.height==other.height
        }
    }

    let rect1=Rectangle{width:30,height:50};
    let rect2 = Rectangle { width: 30, height: 50 };
    let rect3 = Rectangle { width: 40, height: 50 };
    
    println!("矩形1: {}", rect1);
    println!("矩形1 == 矩形2: {}", rect1 == rect2);
    println!("矩形1 == 矩形3: {}", rect1 == rect3);
}
fn main() {
    println!("Hello, world!");
}
