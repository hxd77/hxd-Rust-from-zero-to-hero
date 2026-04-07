use std::any::Any;
use std::arch::x86_64::_mm_pause;
use std::ops::Deref;
use utils::*;
use colored::Colorize;

pub fn run(){
    print_section_title("第15章：智能指针");

    // Box<T> - 堆上数据分配
    box_smart_pointer();

    // Deref trait - 解引用
    deref_trait();

    // Drop trait - 析构
    drop_trait();

    // Rc<T> - 引用计数
    rc_smart_pointer();

    // RefCell<T> - 内部可变性
    refcell_smart_pointer();

    // Rc<T> 和 RefCell<T> 结合
    rc_refcell_combination();

    // 循环引用和内存泄漏
    circular_references();

    // Weak<T> - 弱引用
    weak_references();

    // 线程安全的智能指针
    thread_safe_smart_pointers();

    // 自定义智能指针
    custom_smart_pointers();
}

fn box_smart_pointer(){
    print_example_title("15.1 Box<T> - 堆上数据分配");

    // Box基础
    box_basics();

    // 递归类型
    recursive_types();

    // 大型数据移动
    large_data_movement();

    pause();
}

fn box_basics(){
    println!("\n{}", "Box基础：".blue().bold());

    //基本的Box使用
    let b=Box::new(5);
    println!("Box中的值: {}",b);

    //Box在堆上分配数据
    let heap_value=Box::new(String::from("在堆上的字符串"));
    println!("堆上的字符串: {}",heap_value);

    //Box自动解引用
    let boxed_number=Box::new(42);
    let number=*boxed_number;
    println!("解引用Box: {}",number);

    //Box的所有权转移
    let box1=Box::new(vec![1,2,3,4,5]);
    let box2=box1;//box1的所有权转移给box2
    //println!("box1: {:?}",box1); //编译错误,box1已被移动
    println!("box2: {:?}",box2);

    println!("Box适用于: ");
    println!("- 编译时大小未知的类型");
    println!("- 大量数据的所有权转移");
    println!("- 只关心类型实现特定trait而不是具体类型");
}

fn recursive_types(){
    println!("\n{}", "递归类型".blue().bold());

    //链表实现
    #[derive(Debug)]
    enum List{
        Cons(i32,Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list=Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    println!("递归链表: {:?}", list);

    //二叉树实现
    #[derive(Debug)]
    struct TreeNode{
        value:i32,
        left:Option<Box<TreeNode>>,
        right:Option<Box<TreeNode>>,
    }

    impl TreeNode{
        fn new(value:i32)->Self{
            TreeNode{
                value,
                left:None,
                right:None,
            }
        }

        fn insert(&mut self,value:i32){
            if value<self.value{
                match &mut self.left {
                    Some(node)=>node.insert(value),
                    None=>self.right=Some(Box::new(TreeNode::new(value))),
                }
            }
        }

        fn inorder(&self,result:&mut Vec<i32>){
            if let Some(left)=&self.left {
                left.inorder(result);
            }
            result.push(self.value);
            if let Some(right)=&self.right {
                right.inorder(result);
            }
        }
    }

    let mut root=TreeNode::new(5);
    root.insert(3);
    root.insert(7);
    root.insert(1);
    root.insert(9);

    let mut values=Vec::new();
    root.inorder(&mut values);
    println!("二叉搜索树中序遍历 {:?}", values);
}

fn large_data_movement(){
    println!("\n{}","大型数据移动: ".blue().bold());

    //大型结构体
    struct LargeStruct{
        data:[u8;1024*1024],// 1MB数据
        id:u32,
    }

    impl LargeStruct{
        fn new(id:u32)->Self{
            LargeStruct{
                data:[0;1024*1024],
                id,
            }
        }
    }

    //直接移动会很慢（栈上1MB数据复制）
    fn process_large_struct_direct(s:LargeStruct)->u32{
        s.id
    }

    //使用Box避免大型数据复制
    fn process_large_struct_box(s:Box<LargeStruct>)->u32{
        s.id
    }

    let large1=LargeStruct::new(1);
    let large2=Box::new(LargeStruct::new(2));

    println!("处理直接结构体: {}",process_large_struct_direct(large1));
    println!("处理Box结构体: {}",process_large_struct_box(large2));

    println!("Box只移动指针（8字节），而不是整个结构体");
}

fn deref_trait(){
    print_example_title("15.2 Deref Trait - 解引用");

    //Deref trait基础
    deref_basics();

    //自定义智能指针
    custom_smart_pointer_deref();

    //Deref强制转换
    deref_coercion();

    pause();
}

fn deref_basics(){
    println!("\n{}","Deref trait基础: ".blue().bold());

    //常规解引用
    let x=5;
    let y=&x;

    assert_eq!(5,x);
    assert_eq!(5,*y);
    println!("常规解引用: {} == {}",x,*y);

    //Box的解引用
    let x=5;
    let y=Box::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
    println!("Box解引用: {} == {}",x,*y);

    //Deref trait让*操作符调用deref方法
    println!("*y 实际上是 *(y.deref())");
}

fn custom_smart_pointer_deref(){
    println!("\n{}","自定义智能指针: ".blue().bold());

    //自定义智能指针
    struct MyBox<T>(T); //包含一个元素的元组结构体

    impl<T> MyBox<T>{
        fn new(x:T)->MyBox<T>{
            MyBox(x)
        }
    }

    impl<T>Deref for MyBox<T>{
        type Target=T;

        fn deref(&self)->&Self::Target{
            &self.0
        }
    }

    let x=5;
    let y=MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);
    println!("自定义智能指针解引用: {} == {}",x,*y);

    //不实现Deref会导致编译错误
    //let z=MyBox::new(10);
    // println!("{}", *z); // 没有Deref trait会编译失败

}

fn deref_coercion(){
      println!("\n{}","Deref强制转换: ".blue().bold());

    //MyBox实现Deref
    struct MyBox<T>(T);

    impl<T> MyBox<T>{
        fn new(x:T)->MyBox<T>{MyBox(x)}
    }

    impl <T> Deref for MyBox<T>{
        type Target=T;
        fn deref(&self)->&Self::Target{&self.0}
    }

    fn hello(name:&str){println!("Hello, {}! ",name);}

    //Deref强制转换:&MyBox<String> -> &String -> &str
    let m=MyBox::new(String::from("Rust"));
    hello(&m); //&MyBox<String>自动转换为&str

    // 手动解引用（等价）
    hello(&(*m)[..]);

    println!("Deref强制转换规则:");
    println!("- &T -> &U 当 T: Deref<Target=U>");
    println!("- &mut T -> &mut U 当 T: DerefMut<Target=U>");
    println!("- &mut T -> &U 当 T: Deref<Target=U>");
}

fn drop_trait(){
    print_example_title("15.3 Drop Trait - 析构");

    //Drop trait基础
    drop_basics();

    //提前释放
    early_drop();

    pause();
}

fn drop_basics(){
    println!("\n{}","Drop trait基础: ".blue().bold());

    struct CustomSmartPointer{
        data: String,
    }

    impl Drop for CustomSmartPointer{
        fn drop(&mut self){println!("释放 CustomSmartPointer 数据: `{}`",self.data);}
    }

    {
        let _c = CustomSmartPointer {
            data: String::from("我的数据")
        };
        let _d = CustomSmartPointer {
            data: String::from("其他数据")
        };
        println!("CustomSmartPointer创建完成");
    }// _d和_c在这里离开作用域，按相反顺序调用drop

    println!("作用域结束后继续执行");

    //Drop trait的重要性
    println!("\nDrop trait用于:");
    println!("- 释放内存");
    println!("- 关闭文件");
    println!("- 释放网络连接");
    println!("- 释放锁");
}

fn early_drop(){
    println!("\n{}","提起释放: ".blue().bold());

    struct CustomSmartPointer{
        data:String,
    }
    impl Drop for CustomSmartPointer{
        fn drop(&mut self){println!("释放数据: `{}`",self.data);}
    }

    let c =CustomSmartPointer{
        data:String::from("一些数据"),
    };

    println!("创建了CustomSmartPointer");

    // 不能直接调用drop方法
    // c.drop(); // 编译错误

    // 使用std::mem::drop提前释放
    drop(c);
    println!("在main结束前释放了CustomSmartPointer");

    // 实际使用场景：提前释放锁
    println!("\n提前释放的使用场景:");
    println!("- 释放文件句柄");
    println!("- 释放网络连接");
    println!("- 释放互斥锁");
    println!("- 减少内存使用");
}

fn rc_smart_pointer(){
    print_example_title("15.3 Rc<T> - 引用计数");

    //Rc基础
    rc_basics();

    //共享数据结构
    shared_data_structures();

    pause();
}
#[allow(dead_code)]
fn main(){
    run();
}