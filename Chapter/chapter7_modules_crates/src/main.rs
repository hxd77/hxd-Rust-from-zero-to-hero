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

fn packages_and_crates()
{
    print_example_title("7.5 包和crate");

    //包结构
    packages_structure();

    //二进制和库crate
    binary_and_library_crates();

    //工作空间
    workspaces();

    pause();
}

fn packages_structure()
{
    print!("\n{}","包结构".blue().bold());

    println!("典型的Rust包结构");
    println!("my_package/");
    println!("├── Cargo.toml");
    println!("├── src/");
    println!("│   ├── main.rs        # 二进制crate入口");
    println!("│   ├── lib.rs         # 库crate入口");
    println!("│   └── bin/           # 额外二进制文件");
    println!("│       └── helper.rs");
    println!("├── tests/             # 集成测试");
    println!("│   └── integration_test.rs");
    println!("├── examples/          # 示例");
    println!("│   └── basic_usage.rs");
    println!("└── benches/           # 基准测试");
    println!("    └── benchmark.rs");
    
    println!("\n包和crate概念:");
    println!("- 包(Package): Cargo.toml + 源代码");
    println!("- Crate: 编译单元");
    println!("- 二进制crate: 可执行程序");
    println!("- 库crate: 代码库");
}

fn binary_and_library_crates()
{
    println!("\n{}","二进制和库crate: ".blue().bold());

    //模拟库crate功能
    mod my_library{
        use std::result;

        pub struct Calulator;

        impl Calulator {
            pub fn add(a:i32,b:i32)->i32{
                a+b
            }

            pub fn multiply(a:i32,b:i32)->i32{
                a*b
            }
        }

        pub mod utils{
            pub fn format_results(result:i32)->String{
                format!("计算结果: {}",result) //等价let s = String::from("计算结果: 42");
            }
        }
    }
    //使用库功能
    let result=my_library::Calulator::add(5,3);
    println!("{}",my_library::utils::format_results(result));

    let result=my_library::Calulator::multiply(4, 7);
    println!("{}",my_library::utils::format_results(result));
    
    println!("\nCargo.toml示例:");
    println!("[package]");
    println!("name = \"my_package\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!("");
    println!("[dependencies]");
    println!("serde = \"1.0\"");
    println!("tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    
    println!("\ncrate类型:");
    println!("- bin: 可执行文件");
    println!("- lib: 库文件");
    println!("- 混合: 同时包含");
}

fn workspaces()
{
    println!("\n{}", "工作空间：".blue().bold());
    
    println!("工作空间结构:");
    println!("my_workspace/");
    println!("├── Cargo.toml         # 工作空间配置");
    println!("├── Cargo.lock");
    println!("├── frontend/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── main.rs");
    println!("├── backend/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── main.rs");
    println!("└── shared/");
    println!("    ├── Cargo.toml");
    println!("    └── src/");
    println!("        └── lib.rs");
    
    println!("\n工作空间Cargo.toml:");
    println!("[workspace]");
    println!("members = [");
    println!("    \"frontend\",");
    println!("    \"backend\",");
    println!("    \"shared\",");
    println!("]");
    
    println!("\n工作空间优势:");
    println!("- 统一依赖管理");
    println!("- 共享Cargo.lock");
    println!("- 批量操作");
    println!("- 代码共享");
}

fn filesystem_organization()
{
    print_example_title("7.6 文件系统组织");

    //模块文件分离
    module_file_separation();

    //目录结构最佳实践
    directory_structure_best_practices();

    pause();
}

fn module_file_separation()
{
    println!("\n{}","模块文件分离: ".blue().bold());

    println!("模块组织方式:");
    println!("1. 单文件模块:");
    println!("   src/");
    println!("   ├── lib.rs");
    println!("   └── network.rs      # mod network;");
    
    println!("\n2. 目录模块:");
    println!("   src/");
    println!("   ├── lib.rs");
    println!("   └── network/");
    println!("       ├── mod.rs       # 模块入口");
    println!("       ├── server.rs");
    println!("       └── client.rs");
    
    println!("\n3. 现代组织方式:");
    println!("   src/");
    println!("   ├── lib.rs");
    println!("   ├── network.rs       # 模块入口");
    println!("   └── network/");
    println!("       ├── server.rs");
    println!("       └── client.rs");
    
    println!("\n模块声明:");
    println!("// 在lib.rs或main.rs中");
    println!("mod network;              // 引入network.rs");
    println!("mod database {{           // 内联模块");
    println!("    pub fn connect() {{");
    println!("        // 连接数据库");
    println!("    }}");
    println!("}}");
}

fn directory_structure_best_practices()
{
    println!("\n{}","目录结构最佳实践: ".blue().bold());

     println!("推荐项目结构:");
    println!("my_project/");
    println!("├── Cargo.toml");
    println!("├── README.md");
    println!("├── LICENSE");
    println!("├── src/");
    println!("│   ├── main.rs          # 或 lib.rs");
    println!("│   ├── config/          # 配置相关");
    println!("│   │   ├── mod.rs");
    println!("│   │   └── settings.rs");
    println!("│   ├── models/          # 数据模型");
    println!("│   │   ├── mod.rs");
    println!("│   │   ├── user.rs");
    println!("│   │   └── post.rs");
    println!("│   ├── handlers/        # 请求处理");
    println!("│   │   ├── mod.rs");
    println!("│   │   └── api.rs");
    println!("│   └── utils/           # 工具函数");
    println!("│       ├── mod.rs");
    println!("│       └── helpers.rs");
    println!("├── tests/               # 集成测试");
    println!("├── examples/            # 使用示例");
    println!("└── docs/                # 文档");
    
    println!("\n组织原则:");
    println!("- 按功能分组");
    println!("- 保持层次清晰");
    println!("- 避免过深嵌套");
    println!("- 使用有意义的名称");
}

fn practical_examples(){
    println!("7.7 实际应用案例");

    //库设计案例
    library_design_example();

    //API模块组织
    api_module_organization();

    pause();
}
fn library_design_example() {
    println!("\n{}", "库设计案例：".blue().bold());
    
    // 设计一个简单的HTTP客户端库
    mod http_client {
        pub mod request {
            pub struct HttpRequest {
                pub url: String,
                pub method: String,
                headers: std::collections::HashMap<String, String>,
            }
            
            impl HttpRequest {
                pub fn new(method: &str, url: &str) -> Self {
                    HttpRequest {
                        url: url.to_string(),
                        method: method.to_string(),
                        headers: std::collections::HashMap::new(),
                    }
                }
                
                pub fn add_header(&mut self, key: &str, value: &str) {
                    self.headers.insert(key.to_string(), value.to_string());
                }
                
                pub fn get_headers(&self) -> &std::collections::HashMap<String, String> {
                    &self.headers
                }
            }
        }
        
        pub mod response {
            pub struct HttpResponse {
                pub status: u16,
                pub body: String,
            }
            
            impl HttpResponse {
                pub fn new(status: u16, body: String) -> Self {
                    HttpResponse { status, body }
                }
                
                pub fn is_success(&self) -> bool {
                    self.status >= 200 && self.status < 300
                }
            }
        }
        
        pub mod client {
            use super::{request::HttpRequest, response::HttpResponse};
            
            pub struct Client {
                base_url: String,
            }
            
            impl Client {
                pub fn new(base_url: &str) -> Self {
                    Client {
                        base_url: base_url.to_string(),
                    }
                }
                
                pub fn send(&self, request: &HttpRequest) -> HttpResponse {
                    // 模拟发送请求
                    println!("发送 {} 请求到: {}{}", request.method, self.base_url, request.url);
                    
                    for (key, value) in request.get_headers() {
                        println!("Header: {}: {}", key, value);
                    }
                    
                    HttpResponse::new(200, "响应内容".to_string())
                }
            }
        }
        
        // 便捷函数
        pub fn get(url: &str) -> response::HttpResponse {
            let client = client::Client::new("https://api.example.com");
            let request = request::HttpRequest::new("GET", url);
            client.send(&request)
        }
    }
    
    // 使用库
    let client = http_client::client::Client::new("https://jsonplaceholder.typicode.com");
    let mut request = http_client::request::HttpRequest::new("GET", "/posts/1");
    request.add_header("Accept", "application/json");
    
    let response = client.send(&request);
    println!("响应状态: {}", response.status);
    println!("响应成功: {}", response.is_success());
    
    // 使用便捷函数
    let simple_response = http_client::get("/users/1");
    println!("简单请求状态: {}", simple_response.status);
    
    println!("\n库设计考虑:");
    println!("- 清晰的公共API");
    println!("- 隐藏实现细节");
    println!("- 提供便捷函数");
    println!("- 合理的模块划分");
}


fn api_module_organization() {
    println!("\n{}", "API模块组织：".blue().bold());
    
    // Web API模块结构
    mod web_api {
        pub mod handlers {
            pub mod users {
                pub fn get_user(id: u32) -> String {
                    format!("用户信息: ID {}", id)
                }
                
                pub fn create_user(name: &str) -> String {
                    format!("创建用户: {}", name)
                }
                
                pub fn update_user(id: u32, name: &str) -> String {
                    format!("更新用户 {}: {}", id, name)
                }
            }
            
            pub mod posts {
                pub fn get_posts() -> Vec<String> {
                    vec!["文章1".to_string(), "文章2".to_string()]
                }
                
                pub fn create_post(title: &str, content: &str) -> String {
                    format!("创建文章: {} - {}", title, content)
                }
            }
        }
        
        pub mod middleware {
            pub fn auth_middleware() {
                println!("认证中间件");
            }
            
            pub fn logging_middleware() {
                println!("日志中间件");
            }
        }
        
        pub mod models {
            #[derive(Debug)]
            pub struct User {
                pub id: u32,
                pub name: String,
                pub email: String,
            }
            
            impl User {
                pub fn new(id: u32, name: String, email: String) -> Self {
                    User { id, name, email }
                }
            }
            
            #[derive(Debug)]
            pub struct Post {
                pub id: u32,
                pub title: String,
                pub content: String,
                pub author_id: u32,
            }
        }
        
        pub mod routes {
            use super::handlers;
            use super::middleware;
            
            pub fn setup_routes() {
                println!("设置路由:");
                
                middleware::auth_middleware();
                middleware::logging_middleware();
                
                // 用户路由
                println!("GET /users/:id -> {}", handlers::users::get_user(1));
                println!("POST /users -> {}", handlers::users::create_user("新用户"));
                
                // 文章路由
                let posts = handlers::posts::get_posts();
                println!("GET /posts -> {:?}", posts);
                println!("POST /posts -> {}", handlers::posts::create_post("新文章", "内容"));
            }
        }
    }
    
    // 初始化API
    web_api::routes::setup_routes();
    
    // 创建模型实例
    let user = web_api::models::User::new(1, "张三".to_string(), "zhang@example.com".to_string());
    println!("创建用户: {:?}", user);
    
    let post = web_api::models::Post {
        id: 1,
        title: "Rust模块系统".to_string(),
        content: "详细介绍Rust的模块系统".to_string(),
        author_id: user.id,
    };
    println!("创建文章: {:?}", post);
    
    println!("\nAPI组织原则:");
    println!("- handlers: 请求处理逻辑");
    println!("- models: 数据模型定义");
    println!("- middleware: 中间件功能");
    println!("- routes: 路由配置");
    println!("- utils: 辅助工具");
}

#[allow(dead_code)]//即使这段代码现在没被用到，也不要报“未使用代码”的警告。
fn main() {
    run();
}
