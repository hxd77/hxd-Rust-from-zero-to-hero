use utils::*;
use std::{collections::HashMap, vec};
use colored::Colorize;

pub fn run()
{
    print_section_title("第8章: 常用集合类型");

    //Vector
    vector_examples();

    //字符串
    string_examples();

    //HashMap
    hash_map_exapmles();

    //综合练习
    comprehensive_exercises();
}

fn vector_examples()
{
    print_example_title("8.1 Vector");

    //创建Vector
    creating_vectors();

    //更新Vector
    updating_vectors();

    //读取Vector元素
    reading_vectors();

    //遍历Vector
    iteraing_vectors();

    //使用枚举存储多种类
    sum_in_vector();

    pause();
}
fn creating_vectors(){
    println!("\n{}","创建Vector: ".blue().bold());

    //创建空Vector
    let v:Vec<i32>=Vec::new();
    println!("空Vector: {:?}",v);

    //使用vec!泓创建包含初始值的Vector
    let v=vec![1,2,3];
    println!("使用vec!宏创建: {:?}",v);

    //创建带有初始容量的Vector
    let mut v:Vec<i32>=Vec::with_capacity(10);
    println!("带有初始容量的Vector: {:?}",v);
    println!("容量: {}",v.capacity());
}

fn updating_vectors(){
    println!("\n{}","更新Vector: ".blue().bold());

    let mut v=Vec::new();
    
    //添加元素
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("添加元素后: {:?}",v);

    // 弹出元素
    match v.pop() {
        Some(value) => println!("弹出的元素: {}", value),
        None => println!("Vector为空"),
    }

    println!("弹出后: {:?}",v);

    //插入元素
    v.insert(1, 42);
    println!("插入元素后: {:?}",v);

    //删除元素
    let removed=v.remove(1);
    println!("删除的元素: {}",removed);
    println!("删除后: {:?}",v);

    //清空Vector
    v.clear();
    println!("清空后: {:?}",v);
}

fn reading_vectors(){
    println!("\n{}","读取Vector元素: ".blue().bold());

    let v=vec![1,2,3,4,5];
    
    //使用索引
    let third=&v[2];
    println!("第三个元素: {}",third);

    //使用get方法
    let third=v.get(2);
    match v.get(2){
        Some(third)=>println!("第三个元素: {}",third),
        None=>println!("没有第三个元素"),
    }

    //超出范围的索引
    let does_not_exist=v.get(100);
    println!("不存在的索引: {:?}",does_not_exist);

    //注意: 下面的代码会导致panic
    //let does_not_exist=&v[100];
    
    //不可变引用
    let mut v=vec![1,2,3,4,5];
    let first=&v[0]; //不可变引用
    //v.push(6);//这会导致编译错误
    println!("第一个元素: {}",first);

}

fn main()
{
    run();
}