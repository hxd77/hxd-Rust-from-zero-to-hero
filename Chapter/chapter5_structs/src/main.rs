use utils::*;
use colored::Colorize;

pub fn run()
{
    print_section_title("第5章: 结构体");

    //结构体
    structs_basics();

    //结构体方法
    struct_methods();
}

fn structs_basics()
{
    print_example_title("5.1 结构体基础");

    //定义和实例化结构体
    struct_definition();

    //使用字段初始化简写语法
    field_init_shorthand();

    //结构体更新语法
    struct_update_syntax();

    //元组结构体
    tuple_structs();

    //单元结构体
    unit_structs();

    //结构体数据的所有权
    struct_ownership();

    pause();
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

fn struct_update_syntax()
{
    println!("\n{}","结构体更新语法: ".blue().bold());

    let user1=User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };

    let user2=User{
        email:String::from("another@example.com"),
        ..user1 //其余字段使用user1的值
    };

    println!("用户2的用户名: {}",user2.username);
    println!("用户2的邮箱: {}",user2.email);

    //注意: user1.username和user1.email已经被移到user2
    //println!("用户1的用户名: {}",user1.username);//这会导致编译错误,但是可以使用user1的active和sign_in_count,因为他们是Copy trait类型
}

fn tuple_structs()
{
    println!("\n{}","元组结构体: ".blue().bold());

    //定义元组结构体
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black=Color(0,0,0);
    let origin=Point(0,0,0);

    println!("黑色: ({},{},{})",black.0,black.1,black.2);
    println!("原点: ({},{},{})",origin.0,origin.1,origin.2);

    //虽然两个结构体都有相同的字段类型，但它们是不同的类型
    //let color:Color=origin;这会导致编译错误
}

fn unit_structs()
{
    println!("\n{}","单元结构体: ".blue().bold());

    struct AlwaysEqual;

    let subject=AlwaysEqual;

    println!("创建了一个单元结构体实例");

    //单元结构体在实现trait时很有用
    //例如，可以为AlwaysEqual实现某个trait
}

fn struct_ownership()
{
    println!("\n{}","结构体数据的所有权: ".blue().bold());

    //这里我们使用了String而不是&str;
    //这样结构体实例拥有其所有数据
    let user=User{
        username:String::from("someusername123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
        active:true,
    };

    println!("用户拥有自己的数据: {}",user.username);

    //如果想要在结构体中存储引用，需要使用生命周期
    //这会在后面的章节中介绍
}

fn struct_methods()
{
    print_example_title("5.2 结构体方法: ");

    //定义方法
    method_example();

    //关联函数
    associated_funcion_example();

    //多个impl块
    multiple_impl_blocks();

    pause();
}

//自动为这个结构体、枚举或联合体生成 Debug trait 的实现（结构体本身默认不能打印）
#[derive(Debug)]
//矩形结构体
struct Rectangle{
    width:u32,
    height:u32,
}

//impl方法
impl Rectangle//这个块中的所有内容斗将与Rectangle类型相关联
{
    //方法，与函数不同，它们在结构体的上下文定义
    fn area(&self)->u32{//第一个参数总是self，代表调用该方法的结构体实例
        self.width*self.height
    }

    fn width(&self)->bool{//可以选择将方法的名称与结构体中的一个字段相同
        self.width>0
    }

    fn can_hold(&self,other:&Rectangle)->bool
    {
        self.width>other.width&&self.height>other.height
    }

    //关联函数,所有在impl块中定义的函数被称为关联函数
    fn square(size:u32)->Self{
        Self { width:size, height: size }
    }
}

fn method_example()
{
    println!("\n{}","方法示例: ".blue().bold());

    let rect1=Rectangle{
        width:30,
        height:50,
    };

    println!("矩形: {:?}",rect1);
    println!("面积: {}",rect1.area());

    if rect1.width(){
        println!("矩形的宽度不为零: {}",rect1.width);
    }

    let rect2=Rectangle{
        width:10,
        height:45,
    };

    println!("rect1能容纳rect2: {}",rect1.can_hold(&rect2));
    println!("rect2能容纳rect1: {}",rect2.can_hold(&rect1));
}

fn associated_funcion_example()
{
    println!("\n{}","关联函数示例: ".blue().bold());

    let sq=Rectangle::square(3);//使用结构体名和::语法来调用这个关联函数
    println!("正方形: {:?}",sq);
    println!("正方形面积: {}",sq.area());
}

//可以有多个impl块
impl Rectangle{
    fn perimeter(&self)->u32{
        2*(self.width+self.height)
    }
}

fn multiple_impl_blocks(){
    println!("\n{}","多个impl块示例: ".blue().bold());

    let rect=Rectangle{
        width:30,
        height:50,
    };

    println!("矩形周长: {}",rect.perimeter());
}
fn main() {
    run();
}
