use crate::utils::*;//引入crate（项目）中utils模块里的所有公开项 
use colored::Colorize;//引入colored库的Colorize trait，用于给终端输出添加颜色
pub fn run()//pub表示这个函数是公共的，可以被其他模块调用
{
    print_section_title("第四章：所有权系统");

    //所有权
    owenership_basics();

    //引用和借用
    references_and_borrowing();

    //切片类型
    slice_type();

}

fn ownership_basics(){
    print_example_title("4.1 所有权基础");

    //所有权规则
    ownership_rules();

    //变量作用域
    variable_scope();


}

fn owenership_rules()
{
    println!("\n{}","所有权规则：".blue().bold());
}