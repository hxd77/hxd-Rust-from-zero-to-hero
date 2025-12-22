use utils::*;
use colored::Colorize;

pub fn run(){
    print_section_title("第13章: 函数式编程 - 闭包和迭代器");

    //闭包
    closures();

    //闭包类型推断和标注
    closure_type_inference();

    //move闭包
    move_closures();

    //
}

fn closures(){
    print_example_title("13.1 闭包: 匿名函数");

    //闭包基础
    closure_basics();

    //闭包语法
    closure_syntax();

    //闭包作为参数
    closures_as_parameters();

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
    
}
fn main() {
    println!("Hello, world!");
}
