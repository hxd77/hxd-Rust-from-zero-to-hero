use std::vec;
use std::thread;
use utils::*;
use colored::Colorize;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

pub fn run(){
    print_section_title("第13章: 函数式编程 - 闭包和迭代器");

    //闭包
    closures();

    //闭包类型推断和标注
    closure_type_inference();

    //move闭包
    move_closures();

    //迭代器
    iterator();
    
    //迭代器适配器
    iterator_adaptors();
    
    //消费适配器
    consuming_adaptors();
    
    //函数式编程实践
    functional_programming_pracitce();
    
    //性能对比
    

}

fn closures(){
    print_example_title("13.1 闭包: 匿名函数");

    //闭包基础
    closure_basics();

    //闭包语法
    closure_syntax();

    //捕获环境语法
    capturing_environment();

    //闭包作为参数
    closures_as_parameters();

    //例子-健身计划生成app
    gym_plan_app();

    pause();
}

fn closure_basics(){
    println!("\n{}", "闭包基础：".blue().bold());

    //基本闭包示例
    let add_one=|x|x+1;
    let result=add_one(5);
    println!("5+1={}",result);

    //多参数闭包
    let add=|x,y|x+y;
    println!("3+4 ={}",add(3,4));

    //闭包可以捕获环境
    let multiplier=2;
    let multiply=|x|x*multiplier;
    println!("5 * {} = {}", multiplier, multiply(5));

    //闭包与函数的区别
    fn regular_function(x:i32)->i32{
        x+1
    }

    println!("函数调用: {}", regular_function(5));
    println!("闭包调用: {}", add_one(5));

    println!("闭包可以捕获环境变量，而函数不能");
}

fn closure_syntax(){
    println!("\n{}", "闭包语法：".blue().bold());

    //简单闭包
    let simple=|x|x;
    println!("简单闭包: {}", simple(42));
    
    //带类型注释的闭包
    let with_types=|x:i32|->i32{x+1};
    println!("带类型注释: {}", with_types(41));

    //多行闭包
    let complex=|x:i32|{
        let doubled=x*2;
        let squared=doubled*doubled;
        squared+1
    };
    println!("复杂闭包: {}",complex(3));

    //不接受参数的闭包
    let greeting=||"hello,world";
    println!("无参数闭包: {}",greeting());

    //返回闭包
    fn returns_closure()->impl Fn(i32)->i32{
    |x| x+1
    }

    let returned_closure=returns_closure();
    println!("返回的闭包: {}",returned_closure(10));
}

fn closures_as_parameters(){
    println!("\n{}", "闭包作为参数：".blue().bold());

    //使用Fn trait
    fn call_with_one<F>(f:F)->i32
    where
        F:Fn(i32)->i32, //trait bound
    {
        f(1)
    }

    let double=|x|x*2;
    println!("使用Fn trait: {}", call_with_one(double));

    //使用FnMut trait
    fn call_with_mut<F>(mut f:F)->i32
    where 
        F:FnMut(i32)->i32,
    {
        f(1)+f(2)
    }

    let mut counter=0;
    let incrementer=|x|{ //这是一个闭包，闭包作为参数
        counter+=1;
        x+counter //改变环境变量用FnMut
    };

    println!("使用FnMut trait: {}", call_with_mut(incrementer));

    //使用FnOnce trait
    fn call_once<F>(f:F)->String
    where 
        F:FnOnce()->String
    {
        f()
    }

    let name=String::from("Rust");
    let greeter=||format!("Hello, {}",name); //一个没有参数的闭包
    println!("使用FnOnce trait: {}", call_once(greeter));
}

fn closure_type_inference(){
    print_example_title("13.2 闭包类型推断和标注");

    //类型推断示例
    type_inference_examples();

    //显示类型标注
    explicit_type_annotations();

    pause();
}

fn type_inference_examples(){
    println!("\n{}", "类型推断示例：".blue().bold());

    //Rust会推断闭包的类型
    let add_one=|x| x+1;
    println!("推断为i32: {}", add_one(5i32));
    //一旦使用，类型就被确定了
    // println!("不能用作f64: {}", add_one(5.0f64)); // 这会编译错误

    //不同的调用会导致不同的类型推断
    let to_string_int=|x:i32| format!("{}",x); //后面是返回值
    let to_string_str = |x: &str| format!("{}", x);
    println!("推断类型(整数): {}", to_string_int(42));
    println!("推断类型(字符串): {}", to_string_str("hello"));
    
    //延迟类型推断需要使用泛型
    fn identity<T>(x:T)->T{
        x
    }

    let string_result=identity(String::from("hello"));
    let int_result=identity(42);
    println!("字符串结果: {}", string_result);
    println!("整数结果: {}", int_result);
}

fn explicit_type_annotations(){
    println!("\n{}", "显式类型标注：".blue().bold());

    //完整的类型标注
    let add: fn(i32,i32)->i32=|x,y|x+y;
    println!("函数指针: {}",add(3,4));

    //闭包类型标注
    let multiply=|x:i32,y:i32|->i32{x*y};
    println!("显式标注闭包: {}", multiply(3, 4));

    //泛型闭包
    fn apply_operation<F,T>(f:F,x:T,y:T)->T
    where 
        F:Fn(T,T)->T,
    {
        f(x,y)
    }
    /*
    fn apply_operation<F, T>:
    定义了一个名为 apply_operation 的函数。
    <F, T> 是泛型参数。T 代表某种数据类型（如整数、浮点数），F 代表某种函数或闭包类型。
    (f: F, x: T, y: T) -> T:
    输入参数：一个函数 f，以及两个类型为 T 的操作数 x 和 y。
    返回值：计算结果，类型也是 T。
    where F: Fn(T, T) -> T:
    这是 Trait Bound（约束）。
    它规定了 F 必须是一个“可以接收两个 T 类型参数并返回一个 T 类型结果”的函数或闭包。Fn 是 Rust 中表示函数签名的 Trait。
    f(x, y):
    函数体。调用传入的 f，把 x 和 y 传进去。 */

    let add_generic=|x,y|x+y;
    println!("泛型闭包(整数): {}", apply_operation(add_generic, 5, 3));

    let concat=|x:String,y:String| format!("{}{}",x,y);
    println!("泛型闭包(字符串): {}", apply_operation(concat, "Hello".to_string(), " World".to_string()));
}

fn capturing_environment(){
    print_example_title("13.3 捕获环境");

    //捕获方式
    capturing_modes();

    //Fn, FnMut, FnOnce traits
    clouse_traits();

    pause();
}

fn capturing_modes(){
    println!("\n{}", "捕获方式：".blue().bold());
    
    //不可变借用
    let list=vec![1,2,3];
    println!("调用前: {:?}",list);

    let only_borrows=||println!("从闭包中: {:?}",list);

    println!("调用前: {:?}", list);
    only_borrows();
    println!("调用后: {:?}", list);

    //可变借用
    let mut list=vec![1,2,3];
    println!("调用前: {:?}", list);
    
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("调用后: {:?}", list);

    //获取所有权
    let list=vec![1,2,3];
    println!("调用前: {:?}", list);

    thread::spawn(move || println!("从线程: {:?}", list)) //将数据从主线程转移到子线程
        .join()
        .unwrap();
    
    // println!("调用后: {:?}", list); // 编译错误：list已被移动
}


fn clouse_traits(){
    println!("\n{}", "闭包Traits：".blue().bold());

    //Fn- 不可变借用捕获
    fn call_fn<F>(f:F)
    where 
        F:Fn(),
    {
        f();
        f(); //可以多次调用，只是为了读取
    }

    let x=String::from("hello");
    let fn_clouse=||println!("Fn:{}",x);
    call_fn(fn_clouse);

    //FnMut - 可变借用捕获 可以调用多次，且能修改变量
    fn call_fn_mut<F>(mut f:F)
    where  
        F:FnMut(),
    {
        f();
        f(); //可以多次调用
    }

    let mut y=String::from("world");
    let fn_mut_clouse=||{
        y.push_str("!");
        println!("FnMut: {}",y);
    };
    call_fn_mut(fn_mut_clouse);

    //FnOnce - 获取所有权
    fn call_fn_once<F>(f:F)
    where
        F:FnOnce(),
    {
        f();//只能调用一次
    }

    let z=String::from("rust");
    let fn_once_clouse=move||{
        println!("FnOnce: {}",z);
        drop(z); //消费z
    };
    /*
    let z = String::from("rust"); 在堆上创建了一个字符串变量 z。
    move || { ... } move 关键字强制闭包夺取了变量 z 的所有权。这意味着从这一行开始，z 不再属于主线程，而是被“关”进了闭包的内部空间里。
    drop(z); 这是关键点。drop 函数会显式地销毁变量并释放内存。因为 z 在闭包内部被销毁了，所以这个闭包执行一次之后，它内部的 z 就不存在了。 */
    call_fn_once(fn_once_clouse);
}

fn move_closures(){
    print_example_title("13.4 move闭包");

    //move关键字
    move_key_word_usage();

    //线程中的move闭包
    move_closures_in_threads();

    pause();
}

fn move_key_word_usage(){
    println!("\n{}", "move关键字用法: ".blue().bold());

    let data=vec![1,2,3,4,5];

    //普通闭包（借用）
    let borrow_clouse=||println!("Borrowed: {:?}",data);
    borrow_clouse();
    println!("数据仍然可用: {:?}",data);

    //move闭包(获取所有权)
    let data2=vec![6,7,8,9,10];
    let move_closure=move||println!("Moved: {:?}",data2);
    move_closure();
    // println!("数据不再可用: {:?}", data2); // 编译错误

    //强制move即使不需要
    let x=42;
    let move_copy=move||println!("Moced copy: {}",x);
    move_copy();
    println!("原始值仍然可用: {}",x);//Copy类型的值仍然可用
}

fn move_closures_in_threads(){
    println!("\n{}", "线程中的move闭包：".blue().bold());

    let data=vec![1,2,3,4,5];

    let handle=thread::spawn(move||{
        println!("线程中的数据: {:?}",data);
        data.len()
    });

    let result=handle.join().unwrap();
    println!("线程返回的结果: {}", result);
    
    // 多个线程使用不同的数据
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = vec![i; 5]; // 每个线程获得自己的数据副本
        
        let handle = thread::spawn(move || {
            println!("线程{}的数据: {:?}", i, data);
            data.iter().sum::<i32>()
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.join().unwrap();
        println!("线程结果: {}", result);
    }
    
}

fn gym_plan_app()
{
    print_example_title(("13.5 例子: 健身计划生成app"));

    let simulated_user_specified_value=10;
    let simulated_random_number=7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

}
struct Cacher<T>
    where T:Fn(u32)->u32
{
    calculation:T,
    value:HashMap<u32,u32>,
}
impl<T>Cacher<T>
    where T:Fn(u32)->u32
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new(), //初始化哈希值
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        //缓存逻辑：有则返回，无则计算插入
        *self.value.entry(arg).or_insert_with(||(self.calculation)(arg))
        //or_insert_with用法: 如果键存在，则返回值
        //如果不存在，则用闭包算出一个值存进去
    }
}

fn generate_workout(intensity:u32,random_number:u32){
    let mut expensive_result=Cacher::new(|num| {
        println!("缓慢计算中...");
        thread::sleep(Duration::from_secs(2)); //让线程缓慢执行2s
        num
    });
    if intensity<25{
        println!(
            "今天,做 {} 个俯卧撑!",
            expensive_result.value(intensity)
        );
        println!(
            "然后,做 {} 个仰卧起坐!",
            expensive_result.value(intensity)
        );
    }
    else{
        if random_number==3{
            println!("今天休息一天吧!,记得保持补充水分!");
        }
        else{
            println!(
                "今天,跑步 {} 分钟!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn iterator(){
    print_example_title("13.6 迭代器");

    //迭代器基础
    iterator_basics();

    //Iterator trait和next方法
    iterator_demonstration();

    // 消费迭代器的方法
    iterator_sum();

    //产生其他迭代器的方法
    other_creating_iterators();

    //使用闭包获取环境
    filters_by_size();

    //实现Iterator trait来创建自定义迭代器
    impl_iterator_trait();

    pause();
}

fn iterator_basics(){
    println!("\n{}", "迭代器基础：".blue().bold());

    let v1=vec![1,2,3];

    //for循环中的迭代器
    println!("使用for循环: ");
    for item in &v1{
        println!("  项: {}", item);
    }

    //手动使用迭代器
    println!("\n手动使用迭代器: ");
    let v1_iter=v1.iter();

    for item in v1_iter{
       println!("  项: {}", item);
    }

    //迭代器是惰性的
    let v2=vec![1,2,3];
    let v2_iter=v2.iter();//这里不做任何工作

    println!("\n迭代器是惰性的，直到使用时才开始工作");
    for item in v2_iter{
        println!("  惰性项: {}", item);
    }
}

fn iterator_demonstration(){
    println!("\n{}","Iterator trait和next方法".blue().bold());
    let v1=vec![1,2,3];
    let mut v1_iter=v1.iter();

    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
}

fn iterator_sum() {
    println!("\n{}","消费迭代器的方法".blue().bold());
    let v1=vec![1,2,3];

    let v1_iter=v1.iter();
    let total:i32=v1_iter.sum();
    assert_eq!(total,6);
}


fn other_creating_iterators()
{
    println!("\n{}","产生其他迭代器的方法".blue().bold());

    let v1=vec![1,2,3];
    let v2:Vec<_>=v1.iter().map(|x| x+1).collect();    //map使用闭包来调用每一个元素以生成新的迭代器
    assert_eq!(v2,vec![2,3,4]);
}

#[derive(PartialEq, Debug)]
struct Shoe{
    size:u32,
    style:String,
}

fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()   //into_iter获取v1的所有权并返回拥有所有权的迭代器
        .filter(|s| s.size==shoe_size)
        .collect()  //返回和shoe_size一样的
}

fn filters_by_size(){
    println!("\n{}","使用闭包获取环境".blue().bold());
    let shoes=vec![
        Shoe {size:10,style:String::from("sneaker")},
        Shoe {size:13,style:String::from("sandal")},
        Shoe {size:10,style:String::from("boot")},
    ];

    let in_my_size=shoes_in_my_size(shoes,10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe{size:10,style:String::from("sneaker")},
            Shoe{size:10,style:String::from("boot")},
        ]
    );
}

fn impl_iterator_trait(){
    println!("\n{}","实现Iterator trait来创建自定义迭代器".blue().bold());

    //使用Counter迭代器的next方法
    calling_next_directly();

    //使用自定义迭代器中其他Iterator trait方法
    using_other_iterator_trait_methods();
}

struct Counter{
    count:u32,
}

impl Counter{
    fn new() -> Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter{
    type Item=u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count+=1;
        if self.count<6{
            Some(self.count)
        }
        else{
            None
        }
    }
}

fn calling_next_directly(){
    let mut counter=Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}

fn using_other_iterator_trait_methods(){
    let filter_result: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .inspect(|pair| println!("配对{:?}", pair)) // 边流过边打印
        .map(|(x, y)| x * y)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("filter_result ={:?}", filter_result);
    let sum:u32=Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18,sum);
}
fn main() {
    run();
}
