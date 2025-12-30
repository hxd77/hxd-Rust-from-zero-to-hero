use colored::Colorize;
use utils::*;

pub fn run()
{
    print_section_title("第11章: 自动化测试");

    //如何编写测试
    how_to_write_tests();

    //使用asserts!宏来检查结果
    use_asserts();

    //使用assert_eq!和assert_ne!宏来测试相等
    use_assert_eq_and_assert_ne();


}

pub fn add(left:u64,right:u64)->u64
{
    left+right
}

fn how_to_write_tests()
{
    print_example_title("11.1 如何编写自动化测试");
}

#[cfg(test)]
    mod tests1 {
        use super::*;

        #[test] //表明是一个测试函数
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
#[cfg(test)] //只有挡在运行测试时，这段代码才会被编译和执行
    mod tests2{
        use super::*;

        #[test]
        fn exploration(){
            assert_eq!(2+2,4);
        }

        #[test]
        fn another(){
            panic!("测试失败");
        }
    }

fn use_asserts(){
    print_example_title("11.2 使用assert!宏来检查结果");
    
}
#[derive(Debug)]
pub struct Rectangle
{
    width:u32,
    height:u32,
}

impl Rectangle{
    pub fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width&&self.height>other.height
    }
}

#[cfg(test)]
mod tests3{
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger=Rectangle{width:8,height:7};
        let smaller=Rectangle{width:5,height:1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_canoot_hold_larger(){
        let larger=Rectangle{width:8,height:7};
        let smaller=Rectangle{width:5,height:1};

        assert!(!smaller.can_hold(&larger));
    }
}


fn use_assert_eq_and_assert_ne(){
    print_example_title("11.3 使用assert_eq! 和 assert_ne! 宏来测试相等");
}

pub fn add_two(a:i32)->i32{
    a+2
}

#[cfg(test)]
mod test4{
    use super::*;

    #[test]
    fn it_adds_two(){
        assert_eq!(4,add_two(2));
    }
}