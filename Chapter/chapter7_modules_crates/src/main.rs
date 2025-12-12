use std::path::absolute;

use bank::Account;
use company::hr::{self, Employee};
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

fn use_keyword()
{
    print_example_title("7.3 use关键字");

    //基本use语句
    basic_use_statements();

    //use别名
    use_aliases();

    //重新导出
    re_exports();

    pause();
}

fn basic_use_statements()
{
    println!("\n{}","基本use语句: ".blue().bold());

    mod collections{
        pub mod list{
            pub fn create_vector()
            {
                println!("创建向量");
            }
            
            pub fn sort_vecotr()
            {
                println!("排序向量");
            }
        }
        pub mod map{
            pub fn create_hashmap() {
                println!("创建哈希映射");
            }
        }
    }
    
    //引入特定函数
    use collections::list::create_vector;
    use collections::list::sort_vecotr;
    use collections::map::create_hashmap;

    create_vector();
    sort_vecotr();
    create_hashmap();

    //引入模块
    use collections::list;
    list::create_vector();

    println!("\nuse语句优势");
    println!("- 简化路径");
    println!("- 提高可读性");
    println!("- 避免重复");
}

fn use_aliases()
{
    println!("\n{}","use别名".blue().bold());

    mod graphic{
        pub mod d2{
            pub fn draw_rectangle()
            {
                println!("绘制2D矩形");
            }
        }
        pub mod d3{
            pub fn draw_rectangle(){ //不同模块函数居然可以同名
                println!("绘制3D矩形");
            }
        }
    }

    //使用别名解决命名冲突
    use graphic::d2::draw_rectangle as draw_2d_rect;
    use graphic::d3::draw_rectangle as draw_3d_rect;

    draw_2d_rect();
    draw_3d_rect();

    println!("\n别名用途: ");
    println!("- 解决命名冲突");
    println!("- 缩短长名称");
    println!("- 提高代码清晰度");
}

fn re_exports()
{
    println!("\n{}","重新导出".blue().bold());

    mod api{
        mod internal{
            pub fn process_data(){
                println!("内部数据处理");
            }

            pub fn validate_input(){
                println!("输入验证");
            }
        }

    //重新导出
    pub use internal::process_data;
    pub use internal::validate_input as validate;
    }

    //外部用户可以访问直接使用
    api::process_data();
    api::validate();

    println!("\n重新导出作用: ");
    println!("- 简化公共API");
    println!("- 隐藏内部结构");
    println!("- 提供便捷接口");
}

fn visibility_control(){
    print_example_title("7.4 可见性控制");

    //pub 关键字用法
    pub_keyword_usage();

    //限制可见性
    restricted_visibility();

    pause();
}

fn pub_keyword_usage()
{
    println!("\n{}","pub关键字用法: ".blue().bold());

    mod bank{
        pub struct Account{
            pub id:u32,
            balance:f64, //私有
        }

        impl Account{
            pub fn new(id:u32,initial_balance:f64)->Account{
                Account{
                    id,
                    balance:initial_balance,
                }
            }
        
            pub fn deposit(&mut self,amount:f64) //&mut 可变引用可以修改结构体字段
            {
                self.balance+=amount;
                println!("存款{:2}, 余额: {:2}",amount,self.balance); //{:2}输出占2个宽度
            }

            pub fn get_balance(&self)->f64{
                self.balance
            }

            fn calculate_interest(&self)->f64{
                self.balance*0.5 //私有方法
            }
        }
        
        pub enum TransactionType{
            Deposit,
            Withdrawal,
            Transfer,
        }
    }

    let mut account=bank::Account::new(1001, 1000.0);
    println!("账户ID: {}",account.id);
    account.deposit(500.0);
    println!("当前余额: {:2}",account.get_balance());

    let transaction=bank::TransactionType::Deposit;
    match transaction{
        bank::TransactionType::Deposit=>println!("存款交易"),
        bank::TransactionType::Withdrawal=>println!("取款交易"),
        bank::TransactionType::Transfer=>println!("转账交易"),
    }

    println!("\npub可见性");
    println!("- 结构体: 字段独立控制");
    println!("- 枚举: 变体自动公开");
    println!("- 方法: 独立控制");
}

fn restricted_visibility(){
    println!("\n{}","限制可见性: ".blue().bold());

    mod company{
        pub mod hr{
            pub struct Employee{
                pub name:String,
                pub(crate) id:u32, //字段id只对当前crate内可见
                pub(super) salary:f64, //只对父模块可见
                department: String, //私有
            }

            impl Employee{
                pub fn new(name:String,id :u32,salary:f64,department:String)->Employee{
                    Employee{name,id,salary,department}
                }

                pub fn get_department(&self)->&str{
                    &self.department
                }

                pub(crate) fn get_id(&self)->u32{
                    self.id
                }
            }
        }
       pub fn process_employee(emp:&hr::Employee)
       {
           println!("处理员工: {}",emp.name);
           println!("薪资: {:.2}",emp.salary); //可访问,因为是子模块
       }
    }

    let employee=company::hr::Employee::new(
        "张三".to_string(), 
        12345, 
        50000.0, 
        "工程部".to_string()
    );

    println!("员工姓名: {}",employee.name);
    println!("员工ID: {}",employee.get_id()); //crate内可见
    println!("员工部门: {}",employee.get_department());

    company::process_employee(&employee);

    println!("\n可见性级别");
    println!("- pub: 完全公开");
    println!("- pub(crate): crate内可见");
    println!("- pub(super): 父模块可见");
    println!("- pub(self): 当前模块可见");
    println!("- 无修饰符: 私有");
}


fn main() {
    run();
}
