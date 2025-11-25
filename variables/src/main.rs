use std::io;

fn main() {
    let a=[1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index =String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index:usize=index
        .trim()
        .parse()
        .expect("Index entered was not a number");//转化为usize类型,.trim()去掉首尾空白字符，.parse()尝试将字符串解析为数字
    
    let element = a[index];
    println!("The value of the element at index {} is: {}",index,element);

    another_function(5,'h');
}

fn another_function(value:i32,unit_label:char)
{
    println!("The measurement is: {}{}",value,unit_label);
}
