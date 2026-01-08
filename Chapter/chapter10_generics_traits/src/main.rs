use utils::*;
use colored::Colorize;
use std::collections::btree_map::{IterMut, Values};
use std::fmt::{Display, Pointer, format, write};
use std::ops::Add;
use std::{default, fmt};

pub fn run()
{
    print_section_title("第10章: 泛型和Trait");//其实这章也包含了第19章高级主题大杂烩

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
    returning_trait_types();

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

fn traits_with_generics(){
    println!("\n{}", "trait与泛型结合：".blue().bold());
    
    //定义泛型trait
    trait Container<T>{
        fn get(&self)->&T;
        fn set(&mut self,value:T);
    }

    //为泛型结构体实现trait
    struct Box<T>{
        value:T,
    }

    impl <T> Container<T> for Box<T>{
        fn get(&self)->&T{
            &self.value
        }

        fn set(&mut self,value:T)
        {
            self.value=value;
        }
    }

    let mut int_box=Box{value:42};
    println!("盒子中的值: {}",int_box.get());

    int_box.set(100);
    println!("更新后的值: {}", int_box.get());

    let mut string_box=Box{value:String::from("hello")};
    println!("字符串盒子: {}", string_box.get());
    
    string_box.set(String::from("world"));
    println!("更新后的字符串: {}", string_box.get());
}

fn default_implementations(){
    print_example_title("10.3 默认实现");
    
    // 默认实现示例
    default_implementation_example();
    
    // 默认实现调用其他方法
    default_calling_other_methods();
    
    pause();
}

fn default_implementation_example(){
    println!("\n{}", "默认实现示例：".blue().bold());

    pub trait Summary{
        fn summarize_author(&self)->String;

        fn summarize(&self)->String{
            format!("(阅读更多来自{}的内容...)", self.summarize_author())
        }
     }

     pub struct Tweet{
        pub username:String,
        pub content:String,
        pub reply:bool,
        pub retweet:bool,
     }

     impl Summary for Tweet{
        fn summarize_author(&self)->String {
            format!("@{}", self.username)
        }
     }

     let tweet=Tweet{
        username: "horse_ebooks".to_string(),
        content: "当然，就像你可能知道的那样，人们".to_string(),
        reply: false,
        retweet: false,
     };

     println!("1条新推特: {}", tweet.summarize());
}

fn  default_calling_other_methods(){
    println!("\n{}", "默认实现调用其他方法：".blue().bold());
    
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human{
        fn fly(&self){
            println!("在飞机里飞行");
        }
    }

    impl Wizard for Human{
        fn fly(&self){
            println!("用魔法飞行");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("疯狂挥手");
        }
    }

    let person=Human;

    // 调用不同的fly方法
    person.fly(); // 调用Human的fly方法
    Pilot::fly(&person); // 调用Pilot trait的fly方法
    Wizard::fly(&person); // 调用Wizard trait的fly方法
    
    // 如果没有默认实现，可以这样调用
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}

fn traits_as_parameters(){
    print_example_title("10.4 Trait作为参数");
    
    //impl Trait语法
    impl_trait_syntax();

    //trait bound语法
    trait_bound_syntax();

    pause();
}

fn impl_trait_syntax(){
    println!("\n{}", "impl Trait语法：".blue().bold());

    pub trait Summary{
        fn summarize(&self)->String;
    }

    pub struct NewsArticle{
        pub headline:String,
        pub location:String,
        pub author:String,
        pub content:String,
    }

    impl Summary for NewsArticle{
        fn summarize(&self)->String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet{
        pub username:String,
        pub content:String,
        pub reply:bool,
        pub retweet:bool,
    }

    impl Summary for Tweet{
        fn summarize(&self)->String {
            format!("{}: {}", self.username, self.content)
        }
    }

     // 使用impl Trait作为参数
    pub fn notify(item: &impl Summary) {
        println!("突发新闻！{}", item.summarize());
    }

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

    notify(&article);
    notify(&tweet);
}
fn trait_bound_syntax() {
    println!("\n{}", "trait bound语法：".blue().bold());
    
    trait Summary {
        fn summarize(&self) -> String;
    }
    
    trait Display {
        fn format(&self) -> String;
    }
    
    // 使用trait bound
    pub fn notify<T: Summary>(item: &T) {
        println!("突发新闻！{}", item.summarize());
    }
    
    // 多个trait bound
    pub fn notify_and_display<T: Summary + Display>(item: &T) {
        println!("新闻：{}", item.summarize());
        println!("格式：{}", item.format());
    }

    //使用where子句
    pub fn some_function<T,U>(t:&T,u:&U)->i32
    where 
        T: Display+Clone,
        U: Clone+std::fmt::Debug,
    {
        println!("复杂的函数签名");
        42
    } 

    println!("trait bound语法更适合复杂的约束");
}

fn trait_bounds(){
    print_example_title("10.5 Trait Bound");

    //复杂的trait bound
    complex_trait_bounds();
    
    //条件trait bound 
    conditional_trait_bounds();

    pause();
}

fn complex_trait_bounds(){
    println!("\n{}", "复杂的trait bound：".blue().bold());

    use std::fmt::Display;

    //返回较大值的泛型函数
    fn largest<T:PartialOrd+Copy>(list:&[T])->T{
        let mut largeset=list[0];
        for &item in list{
            if item >largeset{
                largeset=item;
            }
        }
        largeset
    }

    let number_list=vec![34,50,25,100,65];
    let result=largest(&number_list);
    println!("最大的数字是 {}",result);

     // 既能比较又能显示的类型
    fn largest_and_display<T: PartialOrd + Copy + Display>(list: &[T]) -> T {
        let result = largest(list);
        println!("最大值: {}", result);
        result
    }
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    largest_and_display(&char_list);
}

fn conditional_trait_bounds(){
    println!("\n{}", "条件trait bound：".blue().bold());

    use std::fmt::Display;

    struct Pair<T>{
        first:T,
        second:T,
    }

    impl<T>Pair<T>{
        fn new(first:T,second:T)->Self{
            Self{first,second}
        }
    }

    //只有当T实现了Display+PartialOrd时才实现这个方法
    impl<T:Display+PartialOrd> Pair<T>{
        fn cmp_display(&self)->&T{
            if self.first>=self.second{
                println!("较大的值是 {}",self.first);
                &self.first
            }
            else {
                println!("较大的值是 {}",self.second);
                &self.second
            }
        }
    }

    let pair=Pair::new(3,7);
    pair.cmp_display();

    //无条件实现
    println!("所有Pair<T>都有new方法");

    //有条件实现
    println!("只有T实现了Display + PartialOrd的Pair<T>才有cmp_display方法");
}

fn returning_trait_types(){
    print_example_title("10.6 返回实现了trait的类型");

    //返回impl trait
    returning_impl_trait();

    //返回trait对象
    returning_trait_objects();

    pause();
}

fn returning_impl_trait(){
    println!("\n{}", "返回impl Trait：".blue().bold());

    trait Summary{
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username:String,
        content:String,
        reply:bool,
        retweet:bool,
    }

    impl Summary for Tweet{
        fn summarize(&self)->String{
            format!("{}: {}", self.username, self.content)
        }
    }

    //返回实现了Summary trait的类型
    fn returns_summarizable() -> impl Summary{
        Tweet{
            username:String::from("horse_ebooks"),
            content:String::from("当然，就像你可能知道的那样，人们"),
            reply:false,
            retweet:false,
        }
    }

    let tweet=returns_summarizable();
    println!("返回的推特: {}",tweet.summarize());

    // 注意：impl Trait只能返回单一类型
    // 下面的代码会编译失败：
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch { //switch是变量
    //         NewsArticle { ... }
    //     } else {
    //         Tweet { ... }
    //     }
    // }
}

fn returning_trait_objects(){
    println!("\n{}","返回的trait对象: ".blue().bold());

    trait Summary{
        fn summarize(&self) -> String;
    }

    struct Tweet {
        username:String,
        content:String,
        reply:bool,
        retweet:bool,
    }

    impl Summary for Tweet{
        fn summarize(&self) -> String {
            format!("{}: {}",self.username,self.content)
        }
    }

    struct NewsArticle{
        headline:String,
        location:String,
        author:String,
        content:String,
    }

    impl Summary for NewsArticle{
        fn summarize(&self)-> String {
            format!("{}, by {} ({})",self.headline,self.author,self.location)
        }
    }

    //返回trait对象可以返回不同的类型
    fn returns_summarizable(switch:bool)->Box<dyn Summary>{
        if switch{
            Box::new(NewsArticle{
                headline: String::from("Rust发布新版本"),
                location: String::from("全球"),
                author: String::from("Rust Team"),
                content: String::from("Rust 1.70带来了许多新特性..."),
            })
        }
        else {
            Box::new(Tweet
            {
                username: String::from("rust_lang"),
                content: String::from("Rust编程语言官方推特"),
                reply: false,
                retweet: false,
            })
        }
    }

    let item1=returns_summarizable(true);
    let item2=returns_summarizable(false);

    println!("动态分发1: {}",item1.summarize());
    println!("动态分发2: {}",item2.summarize());
}

fn conditional_implementations(){
    print_example_title("10.7 有条件地实现方法");

    println!("\n{}","有条件地实现方法: ".blue().bold());

    use std::fmt::Display;

    //为实现Display trait的类型提供to_string方法
    // 这是标准库中的blanket implementation
    println!("标准库为所有实现了Display trait的类型提供了to_string方法");

    let s=3.to_string();
    println!("数字转字符串: {}", s);

    //自定义blanket implementation毯式实现
    trait MyDisplay{
        fn my_to_string(&self)->String;
    }

    impl<T:Display>MyDisplay for T{
        fn my_to_string(&self)->String{
            format!("{}", self)
        }
    }

    let number=42;
    let text="hello";

    println!("数字: {}", number.my_to_string());
    println!("文本: {}", text.my_to_string());
}
fn advanced_traits() {
    print_example_title("10.10 高级trait特性");

    //关联类型
    associated_types();

    //默认泛型类型参数和运算符重载
    default_T_type_parameter_and_operator_overloading();

    //完全限定语法
    fully_qualified_syntax();

    //父trait
    supertraits();

    //newtype模式
    newtype_pattern();

    pause();
}

fn associated_types(){
    println!("\n{}", "关联类型：".blue().bold());

    trait Iterator{
        type Item;

        fn next(&mut self)->Option<Self::Item>;
    }

    struct Counter{
        current:usize,
        max:usize,
    }

    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }

    impl Iterator for Counter{ //为类型Counter实现标准库里的Iterator trait
        type Item=usize; //关联类型，每次next()返回的元素类型

        fn next(&mut self)->Option<Self::Item> {
            if self.current<self.max{
                let current=self.current;
                self.current+=1;
                Some(current)//如果还有元素返回Option<usize>
            }
            else {
                None
            }
        }
    }

    let mut counter=Counter::new(5);
    while let Some(value)=counter.next(){ //只要模式匹配就一直返回
        println!("计数器值: {}",value);
    }

    println!("关联类型避免了在每次使用trait时都指定类型参数");
}

fn default_T_type_parameter_and_operator_overloading(){
    use std::ops::Add;//这个Add就是一个trait

    #[derive(Debug,PartialEq)]
    struct  Point{
        x:i32,
        y:i32,
    }

    impl Add for Point{
        type Output = Point;
        fn add(self, other: Self) -> Point{
         Point{
            x:self.x+other.x,
            y:self.y+other.y,
            }
        }  
    }

    assert_eq!(Point{x:1,y:0}+Point{x:2,y:3},Point{x:3,y:3});

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters{
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0+(other.0*1000))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
