use colored::Colorize;

pub fn print_section_title(title:&str)//pub表示函数是公有的，可以在模块外调用
{
    println!("\n{}","=".repeat(50).cyan());//repeat重复50次，cyan(青色)
    println!("{}",title.yellow().bold());
    println!("{}","=".repeat(50).cyan());
}

pub fn print_example_title(title: &str)
{
    println!("\n{}", title.green().bold());
    println!("{}", "-".repeat(title.len()).green());
}


/*std::fmt::Display 是 Rust 标准库里的一个 trait（特征），用来表示：
一个类型可以被格式化为给人看的字符串（通过 {} 打印）。 T必须是这样一个类型*/
pub fn print_result<T: std::fmt::Display>(result: T) {

    println!("{}: {}", "结果".blue().bold(), result);
}

pub fn print_error<T:std::fmt::Display>(error:T)
{
    eprintln!("{}:{}","错误".red().bold(),error);
}

pub fn pause()
{
    println!("\n{}","按回车键继续...".magenta().bold());
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).unwrap();
    /*.unwrap()

因为 read_line 返回 Result<usize, std::io::Error>：

读取成功 → Ok(字节数)

读取失败 → Err(错误)

unwrap() 的意思：

如果是 Ok 就拿到里面的值；
如果是 Err 就直接 panic（崩溃），打印错误信息。 */
}

//用于测试的示例数据
pub struct Person //定义结构体，相当于public
{
    pub name :String,
    pub age:u32,
}

impl Person//impl用于为结构体实现方法
{
    pub fn new(name:String,age:u32)->Self//初始化结构体
    {
        Person{name,age}
    }
}

impl std::fmt::Display for Person //给Person实现Display trait,可以用{}打印
{
    fn fmt(&self,f:&mut std::fmt::Formatter)->std::fmt::Result//Display trait要求实现fmt方法
    {
        write!(f,"Person {{ name: {}, age: {} }}",self.name,self.age)//write!宏用于格式化输出到指定的缓冲区，这里是f
    }
}