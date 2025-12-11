use std::path::absolute;

use library::Category;
use utils::*;
use colored::Colorize;

pub fn run()
{
    print_section_title("第7章: 模块系统和包管理");

    //模块基础
    module_basic();

    //路径和引用
    paths_and_reference();

    //use关键字
    use_keyword();

    //可见性控制
    visibility_control();

    //包和crate
    packages_and_crates();

    //文件系统组织
    filesystem_organization();

    //实际应用案例
    practical_examples();
}

fn module_basic()
{
    print_example_title("7.1 模块基础");

    //模块定义
    module_definition();

    //嵌套模块
    nested_modules();

    //模块中的函数和结构体
    functions_and_structs_in_modules();

    pause();
}

fn module_definition()
{
    println!("\n{}","模块定义: ".blue().bold());

    //在函数内定义模块(仅用于演示，一般是cargo new --lib restaurant)
    mod front_of_house{
        pub mod hosting{
            pub fn add_to_waitlist()
            {
                println!("添加到等待列表");
            }

            fn seat_at_table()
            {
                println!("安排座位");
            }
        }
        mod serving
        {
            fn take_order()
            {
                println!("接受订单");
            }

            fn serve_order()
            {
                println!("上菜");
            }

            fn take_payment()
            {
                println!("收款");
            }
        }
    }

//调用模块中的函数
front_of_house::hosting::add_to_waitlist(); //相对路径
//绝对路径crate::front_of_house::hosting::add_to_waitlist();

println!("\n模块特点");
println!("- 使用mod关键字定义");
println!("- 形成树形结构");
println!("- 默认私有");
println!("- 使用pub使其公开");
}

fn nested_modules()
{
    println!("\n{}","嵌套模块: ".blue().bold());

    mod sound{
        pub mod instrument{
            pub fn clarinet(){
                println!("单簧管");
            }

            pub mod woodwind
            {
                pub fn flute()
                {
                    println!("长笛");
                }

                pub fn oboe()
                {
                    println!("双簧管");
                }
            }
        }

        pub mod voice{
            pub fn soprano()
            {
                println!("女高音");
            }

            pub fn alto()
            {
                println!("女低音");
            }
        }
    }

    //调用嵌套模块中的函数
    sound::instrument::clarinet();
    sound::instrument::woodwind::flute();
    sound::voice::soprano();

    println!("\n模块路径");
    println!("- 绝对路径: 从crate根开始");
    println!("- 相对路径: 从当前模块开始");
    println!("- 使用::分隔符");
}

fn functions_and_structs_in_modules()
{
    println!("\n{}","模块中的函数和结构体: ".blue().bold());

    mod library{
        pub struct Book{
            pub title:String,
            author:String,//私有字段
        }
    
        impl Book{
            pub fn new(title:String,author:String)->Book{
                Book{title,author}
            }

            pub fn get_author(&self)->&str{
                &self.author
            }
        }

        pub enum Category{
            Fiction,
            NonFiction,
            Science,
        }

        pub fn create_book(title:&str,author:&str)->Book{
            Book::new(title.to_string(), author.to_string())//把&str转换成String
        }
    }
    
    let book=library::create_book("Rust编程", "Steve Klabnik");
    println!("书名: {}",book.title);
    println!("作者: {}",book.get_author());

    let category=library::Category::Science;
    match category {
        library::Category::Fiction=>println!("小说"),
        library::Category::NonFiction=>println!("非小说"),
        library::Category::Science=>println!("科学"),
    }

    println!("\n模块组织");
    println!("- 结构体字段默认私有");
    println!("- 枚举变体默认公开");
    println!("- 需要构造函数创建实例");
}

fn paths_and_reference(){
    print_example_title("7.2 路径和引用");
    
    //绝对路径和相对路径
    absolute_and_relative_paths();

    //super和self关键字

    pause();
}

fn absolute_and_relative_paths()
{
    println!("\n{}","绝对路径和相对路径: ".blue().bold());

    mod restaurant{
        pub mod front_of_house{
            pub mod hosting{
                pub fn add_to_waitlist()
                {
                    println!("添加到等待列表");
                }
            }
        }

        pub fn eat_at_restaurant()
        {
            //绝对路径
            //crate::front_of_house::hosting::add_to_waitlist();

            //相对路径
            front_of_house::hosting::add_to_waitlist();
        }
    }

    restaurant::eat_at_restaurant();

    println!("\n路径类型");
    println!("- 绝对路径: 从crate开始");
    println!("- 相对路径: 从当前位置开始");
    println!("- 选择原则: 代码移动频率");
}

fn super_and_self_keywords()
{
    println!("\n{}","super和self关键字: ".blue().bold());

    mod math{
        fn basic_operation()
        {
            println!("基础数学运算");
        }

        pub mod advanced{
            pub fn complex_calculation(){
                println!("复杂计算");
                //使用super访问父模块
                super::basic_operation();
            }
    
            pub fn self_reference()
            {
                //使用self引用当前模块
                self::complex_calculation();
            }
        }
    }
    math::advanced::complex_calculation();
    math::advanced::self_reference();

    println!("\n特殊关键字: ");
    println!("- super: 访问父模块");
    println!("- self: 引用当前模块");
    println!("- crate: 引用根模块");
}


fn main() {
    run();
}
