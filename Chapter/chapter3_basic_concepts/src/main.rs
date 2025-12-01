use utils::*;
use colored::{Colorize};

pub fn run(){
    print_section_title("第三章：基础概念");    

    //变量和可变性
    variables_and_mutability();

    //数据类型
    data_types();

    //函数
    functions();

    //注释
    comments();

    //控制流
    control_flow();

}


fn variables_and_mutability()
{
    print_example_title("3.1 变量和可变性");

    //不可变变量
    let x = 5;
    println!("不可变变量 x 的值是: {}",x);

    //可变变量
    let mut y=5;
    println!("可变变量 y 的初始值是: {}",y);

    y=6;
    println!("可变变量 y 的新值是: {}",y);

    //常量至始至终都不可变
    const THREE_HOURS_IN_SECONDS:u32=60*60*3;
    println!("常量 THREE_HOURS_IN_SECONDS 代表 3 个小时的秒数的值是: {}",THREE_HOURS_IN_SECONDS);
    
    //遮蔽（第一个变量会被第二个变量遮蔽）
    let x=5;
    let x=x+1; //重复使用let而不允许直接赋值
    {
        let x=x*2;
        println!("内部作用域中的 x 的值是: {}",x);
    }
    println!("外部作用域中的 x 的值是: {}",x);

    //遮蔽可以改变类型
    let spaces="   ";
    let spaces=spaces.len();
    println!("spaces 变量现在是一个数字，值是: {}",spaces);

    pause();
}

fn data_types()
{
    print_example_title("3.2 数据类型");

    //标量类型
    scalar_types();

    //复合类型
    compound_types();

    pause();
}

fn scalar_types()
{
    println!("\n{}","标量类型: ".blue().bold());

    //整数类型，默认是i32
    let decimal=98_222;
    let hex=0xff;
    let octal=0o77;
    let binary=0b1111_0000;
    let byte=b'A';  //仅限于u8

    println!("整数类型示例: ");
    println!(" 十进制: {}",decimal);
    println!(" 十六进制: {}",hex);
    println!(" 八进制: {}",octal);
    println!(" 二进制: {}",binary); 
    println!(" 字节: {}",byte);

    //浮点类型
    let x=2.0; //f64
    let y: f32=3.0; //f32

    println!("\n浮点类型示例: ");
    println!(" f64的 x 的值是: {}",x);
    println!(" f32的 y 的值是: {}",y);


    //数字运算
    let sum=5+10;
    let difference=95.5-4.3;
    let product=4*30;
    let quotient=56.7/32.2;
    let remainder=43%5;

    println!("\n数字运算示例: ");
    println!(" 加法: {}",sum);
    println!(" 减法: {}",difference);
    println!(" 乘法: {}",product);
    println!(" 除法: {}",quotient);
    println!(" 取余: {}",remainder);

    //布尔类型
    let t=true;
    let f:bool=false;

    println!("\n布尔类型示例: ");
    println!(" t={} ",t);
    println!(" f={}",f);


    //字符类型
    let c='z';
    let z='Z';
    let heart_eyed_cat='😻';

    println!("\n字符类型示例: {}",c);
    println!(" ASCII字符: {}",z);
    println!(" Unicode字符: {}",heart_eyed_cat);
}

fn compound_types()
{
    println!("\n{}","复合类型: ".blue().bold());

    //元组类型 
    println!("元组的长度是固定的,类型可以不同");
    let tup:(i32,f64,u8)=(500,6.4,1);
    let (x,y,z)=tup;

    println!("  \n元组类型示例: ");
    println!("  整个元组: {:?}",tup);//{:?}表示Debug模式打印
    println!("  解构后: x={},y={},z={}",x,y,z);
    println!("  通过索引访问: {}",tup.0);

    //数组类型
    println!("数组也具有固定长度,且类型必须相同");
    let a=[1,2,3,4,5];
    let b:[i32;5]=[1,2,3,4,5];
    let c=[3;5];//[3,3,3,3,3]

    println!("\n数组类型示例");
    println!("  数组a: {:?}",a);
    println!("  数组b: {:?}",b);
    println!("  数组c: {:?}",c);
    println!("  数组长度: {:?}",a.len());
    println!("  第一个元素: {}",a[0]);
    println!("  最后一个元素:   {}",a[a.len()-1]);
}

fn functions()
{
    print_example_title("3.3 函数");

    //调用函数
    another_function();

    //带参数的函数
    function_with_parameter(5);

    //多参数函数
    print_labeled_measurement(5,'h');

    //语句和表达式
    statements_and_expressions();

    //返回值的函数
    let x=five();
    println!("five()的返回值:   {}",x);

    let x=plus_one(5);
    println!("plus_one(5)的返回值:  {}",x);

    pause();
}

fn another_function()
{
    println!("另一个函数");
}

fn function_with_parameter(x:i32)
{
    println!("参数x的值:    {}",x);
}

fn print_labeled_measurement(value:i32,unit_label:char)
{
    println!("测量值:   {}{}",value,unit_label);
}

fn statements_and_expressions()
{
    println!("\n{}","语句和表达式:  ".blue().bold());
    
    //语句
    let y=6;
    println!("语句创建的变量y:  {}",y);

    //表达式
    let x=
    {
        let y=3;
        y+1   //这是一个表达式，没有分号
    };
    println!("表达式的结果x:{}",x);
}

fn five()->i32{
    5//函数的返回值等同于函数体最后一个表达式的值
}

fn plus_one(x:i32)->i32{
    x+1
}


fn comments(){
    print_example_title("3.4 注释");

    //这是一个单行注释
    println!("Hello,world!");

    /*
    这是一个多行注释 
    */
    println!("注释示例已展示");
    pause();
}

fn control_flow(){
    print_example_title("3.5 控制流");

    //if表达式
    if_expressions();

    //循环
    loops();

    pause();
}

fn if_expressions()
{
    println!("\n{}","if表达式: ".blue().bold());

    let number=3;

    if number<5{
        println!("条件为真");
    }
    else {
        println!("条件为假");
    }

    //多重条件
    let number=6;
    
    if number%4==0
    {
        println!("数字能被4整除");
    }
    else if  number%3==0{
        println!("数字能被3整除");
    }
    else if number%2==0
    {
        println!("数字能被2整除");
    }
    else {
        println!("数字不能被4、3、2整除");
    }

    //在let语句中使用if
    let condition=true;
    let number=if condition{5}else {6};

    println!("number的值: {}",number);
}

fn loops()
{
    println!("\n{}","循环: ".blue().bold());

    //loop循环
    loop_example();

    //从循环返回
    loop_return();

    //while循环
    while_loop();

    //for循环
    for_loop();
}

fn loop_example()
{
    println!("\n使用loop重复执行代码: ");

    let mut count=0;
    'counting_up:loop {//循环标签
        println!("count={}",count);
        let mut remaining=10;

        loop {
            println!("remaining={}",remaining);
            if remaining==9{
                break;
            }
            if count ==2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count={}",count);
}

fn loop_return()
{
    println!("\n从循环返回: ");
    let mut counter=0;

    let result=loop {
        counter+=1;

        if counter==10{
            break counter*2;
        }
    };
    
    println!("result={}",result);
}

fn while_loop(){
    println!("\nwhile循环示例:  ");

    let mut number=3;
    
    while number!=0{
        println!("{}!",number);
        number-=1;
    }

    println!("发射!");
}

fn for_loop()
{
    println!("\nfor循环示例:    ");

    //遍历数组
    let a=[10,20,30,40,50];
    
    for element in a.iter(){//iter()只读
        println!("值:   {}",element);
    }

    //使用范围
    for number in(1..4).rev(){//从1-4的反向
        print!("{}!",number);
    }
    println!("发射!");

    //遍历集合
    let names=vec!["Alice","Bob","Charlie"];
    for (index,name) in names.iter().enumerate(){//iter()表示&引用，不拿走所有权，enumerate表示加上索引
        println!("{}.{}",index+1,name);
    }
}

fn main() {
    run();
}
