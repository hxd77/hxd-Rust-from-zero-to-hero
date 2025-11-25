fn main()
{
    for number in (1..4).rev(){ //(1..4)表示1-3的范围，rev()表示反转
        println!("The number is: {}", number);
    }
    println!("LIFTOFF!!!");
}