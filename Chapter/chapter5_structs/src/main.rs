use utils::*;
use colored::Colorize;

pub fn run()
{
    print_section_title("第5章: 结构体");

    //结构体
    structs_basics();

    //结构体方法

}

fn structs_basics()
{
    print_example_title("5.1 结构体基础");

    //定义和实例化结构体
    struct_definition();
}

//用户结构体定义
struct User
{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn struct_definition()
{
    println!("\n{}","定义和实例化结构体: ".blue().bold());

    let user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };
    println!("用户1信息: ");
    println!("\n用户名: {}",user1.username);
    println!("\n邮箱: {}",user1.email);
    println!("\n活跃状态: {}",user1.active);
    println!("\n登录次数: {}",user1.sign_in_count);

    //修改可变结构体的字段
    let mut user2=User{
        email:String::from("another@example.com"),
        username:String::from("anotherusername567"),
        active:true,
        sign_in_count:1,
    };
    user2.email=String::from("anotheremail@example.com");
    println!("\n用户2的新邮箱: {}",user2.email);
}

fn field_init_shorthand()
{
    println!("\n{}","字段初始化简写语法: ".blue().bold());
    
    let email=String::from("user@example.com");
    let username:String=String::from("user123");

    let user=build_user(email,username);
    println!("构建的用户: {} <{}>",user.username,user.email);
}

fn build_user(email:String,username:String)->User{
    User { 
        active: true, 
        username,   //简写：等价于username:username
        email,      //简写：等价于email:email
        sign_in_count: 1, 
    }
}

fn main() {
    run();
}
