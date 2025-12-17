use colored::Colorize;
use utils::*;

pub fn run()
{
    print_section_title("第11章: 自动化测试");

    //如何编写测试
    how_to_write_tests();

    //使用asserts!宏来检查结果
    use_asserts();

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
    #[derive(Debug)]
    struct Rectanle
    {
        width:u32,
        height:u32,
    }
    
}

