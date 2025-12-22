use utils::*;
use std::{collections::HashMap, convert, fmt::format, hash::Hash};
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
    enum_in_vector();

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

fn iteraing_vectors()
{
    println!("\n{}","遍历Vector: ".blue().bold());

    let v=vec![100,32,57];

    //遍历不可变引用
    for i in &v{
        println!(" {}",i);
    }

    //遍历可变引用
    let mut v=vec![100,32,57];
    println!("遍历可变引用");
    for i in &mut v{
        *i+=50;
        println!(" {}",i);
    }

    //遍历拥有所有权
    println!("遍历拥有所有权: ");
    for i in v{
        println!(" {}",i);
    }
    //注意: v在这里已经不能用了
}

fn enum_in_vector(){
    println!("\n{}","使用枚举存储多种类型: ".blue().bold());

    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row=vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("电子表格行: {:?}",row);

    //处理不同类型的值
    for cell in &row{
        match cell{
            SpreadsheetCell::Int(i)=>println!("整数: {}",i),
            SpreadsheetCell::Float(f)=>println!("浮点数: {}",f),
            SpreadsheetCell::Text(s)=>println!("文本: {}",s),
        }
    }
}

fn string_examples(){
    print_example_title("8.2 字符串");

    //创建字符串
    creating_strings();

    //更新字符串
    updating_strings();

    //索引字符串
    indexing_strings();

    //切片字符串
    slicing_strings();

    //遍历字符串
    iterating_strings();

    pause();
}

fn creating_strings(){
    println!("\n{}","创建字符串: ".blue().bold());

    //创建空字符串
    let mut s=String::new();
    println!("空字符串: '{}'",s);

    //从字符串字面量创建
    let data="initial contents";
    println!("从字符串字面量创建: '{}'",s);

    //使用String::from
    let s=String::from("inital contents");
    println!("使用String::from: '{}'",s);

    //包含UTF-8字符的字符串
    let hello=String::from("السلام عليكم");
    println!("阿拉伯语: {}",hello);

    let hello=String::from("Dobrý den");
    println!("捷克语: {}",hello);

     let hello = String::from("Hello");
    println!("英语: {}", hello);
    
    let hello = String::from("שָׁלוֹם");
    println!("希伯来语: {}", hello);
    
    let hello = String::from("नमस्ते");
    println!("印地语: {}", hello);
    
    let hello = String::from("こんにちは");
    println!("日语: {}", hello);
    
    let hello = String::from("안녕하세요");
    println!("韩语: {}", hello);
    
    let hello = String::from("你好");
    println!("中文: {}", hello);
    
    let hello = String::from("Olá");
    println!("葡萄牙语: {}", hello);
    
    let hello = String::from("Здравствуйте");
    println!("俄语: {}", hello);
    
    let hello = String::from("Hola");
    println!("西班牙语: {}", hello);

}

fn updating_strings(){
    println!("\n{}","更新字符串: ".blue().bold());

    //使用push_str追加字符串切片
    let mut s=String::from("foo");
    s.push_str("bar");
    println!("使用push_str: {}",s);

    //push_str不会获取参数的所有权
    let mut s1=String::from("foo");
    let s2="bar";
    s1.push_str(s2);
    println!("s1: {},s2: {}",s1,s2);

    //使用push追加字符
    let mut s=String::from("lo");
    s.push('l');
    println!("使用push: {}",s);

    //使用+连接字符串
    let s1=String::from("Hello, ");
    let s2=String::from("world!");
    let s3=s1+&s2; //注意s1被移动了,不能继续使用
    println!("使用+连接: {}",s3);

    //多个字符串连接
    let s1=String::from("tic");
    let s2=String::from("tac");

    
    //使用format!宏
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    let s=format!("{}-{}-{}",s1,s2,s3);
    println!("使用format!宏: {}",s);
    println!("s1, s2 ,s3仍然可用: {},{},{}",s1,s2,s3);
}

fn indexing_strings(){
    println!("\n{}","索引字符串: ".blue().bold());

    let s1=String::from("hello");
    //let h =s1[0]; //这会导致编译错误

    println!("字符串索引在Rust中是不被允许的");

    //字符串的内部表示
    let hello=String::from("Hola");
    println!("Hola 的长度: {}",hello.len());

    let hello = String::from("Здравствуйте");
    println!("'Здравствуйте'的长度: {}", hello.len());
    
    // 字节、标量值和字形簇
    let hello = "नमस्ते";
    println!("'नमस्ते'的不同视图:");
    println!("  字节: {:?}", hello.as_bytes());
    println!("  字符: {:?}", hello.chars().collect::<Vec<char>>());
    // 字形簇需要外部crate
}

fn slicing_strings(){
    println!("\n{}","切片字符串: ".blue().bold());

    let hello="Здравствуйте";
    let s=&hello[0..4];
    println!("切片结果: {}",s);

    //注意: 必须在字符边界上进行切片
    //let s=&hello[0..1]; //这会导致panic

    println!("字符串切片必须在字符边界上进行");
    
}

fn iterating_strings(){
    println!("\n{}", "遍历字符串：".blue().bold());

    let hello= "नमस्ते";

    //遍历字符
    println!("遍历字符: ");
    for c in hello.chars(){
        println!(" {}",c);
    }

    //遍历字节
    println!("遍历字节:");
    for b in hello.bytes(){
        println!(" {}",b);
    }
}

fn hash_map_exapmles(){
    print_example_title("8.3 HashMap");

    //创建HashMap
    creating_hash_maps();

    //访问HashMap中的值
    accessing_hash_maps();

    //更新HashMap
    updating_hash_maps();

    //遍历HashMap();
    iteraing_hash_maps();

    pause();
}

fn creating_hash_maps(){
    println!("\n{}", "创建HashMap：".blue().bold());

    //创建空HashMap
    let mut scores=HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("分数: {:?}", scores);
    
    //使用collect方法创建HashMap
    let teams=vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores=vec![10,50];

    let mut socre:HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();

     println!("使用collect创建: {:?}", scores);
}

fn accessing_hash_maps(){
    println!("\n{}", "访问HashMap中的值：".blue().bold());

    let mut scores=HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"), 50);

    //使用get方法
    let team_name=String::from("Blue");
    let score=scores.get(&team_name);

    match score{
        Some(score)=>println!("Blue队的分数: {}",score),
        None=>println!("Blue队不存在"),
    }

    //使用get的简化写法
    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("Blue队的分数（使用unwrap_or）: {}", score);
}

fn updating_hash_maps(){
    println!("\n{}", "更新HashMap：".blue().bold());

    //覆盖值
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("覆盖值: {:?}", scores);

    //只在键没有对应值时插入
    let mut scores=HashMap::new();
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);//or_insert会返回这个键的值的一个可变引用(&mut v)

    println!("只在键没有对应值时插入: {:?}", scores);

    //根据旧值更新一个值
    let text="hello world wonderful world";
    let mut map=HashMap::new();

    for word in text.split_whitespace(){
        let count=map.entry(word).or_insert(0);
        *count+=1;
    }

    println!("单词计数: {:?}", map);
}

fn iteraing_hash_maps(){
    println!("\n{}", "遍历HashMap：".blue().bold());

    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);

    //遍历键值对
    println!("遍历键值对: ");
    for(key ,value) in &scores{
        println!("  {}: {}", key, value);
    }

    //遍历键
    println!("遍历键: ");
    for value in scores.values(){
        println!(" {}",value);
    }

    //可变遍历值
    println!("可变遍历值（每个值增加10）:");
    for value in scores.values_mut(){
        *value+=10; //返回一个引用后需要*解引用
    }
    println!("更新后的分数: {:?}", scores);
}

fn comprehensive_exercises(){
    print_example_title("8.4 综合练习");
    
    // 练习1：给定一系列数字，使用vector并返回这个列表的中位数和众数
    median_and_mode();
    
    // 练习2：将字符串转换为猪拉丁文
    pig_latin();
    
    // 练习3：使用HashMap和vector创建一个文本接口来允许用户增加员工名字到公司的部门中
    employee_management();
    
    pause();
}

fn median_and_mode(){
    println!("\n{}", "练习1：中位数和众数".blue().bold());

    let mut numbers=vec![1,2,3,3,4,4,4,5,5,6];
    println!("原始数组: {:?}", numbers);

    //计算中位数
    numbers.sort();
    let median=if numbers.len()%2==0{
        let mid=numbers.len()/2;
        (numbers[mid-1]+numbers[mid])as f64/2.0
    }
    else {
        numbers[numbers.len()/2] as f64
    };

    println!("中位数: {}",median);

    //计算众数
    let mut counts=HashMap::new();
    for &num in &numbers{
        *(counts.entry(num).or_insert(0))+=1;
    }

    let mode=counts.into_iter().max_by_key(|(_,count)|*count).map(|(num,_)|num).unwrap();

    println!("众数: {}",mode);
    /*counts.iter()

    |(_, count)| ... 是一个闭包（匿名函数），用于提取“比较键”

    忽略 key
    
    count → HashMap value 的引用 &i32
    
    *count → 解引用，得到 i32 本身，用作比较键
    
    所以整个闭包的意思是：只看 value，用它来比较大小
    最后返回Some((&key,&value))
    */
}

fn pig_latin(){
    println!("\n{}", "练习2：猪拉丁文转换".blue().bold());

    let words=vec!["first","apple","hello","world"];

    for word in words{
        let pig_latin=convert_to_pig_latin(word);
        println!("'{}' -> '{}' ",word,pig_latin);
    }
}

fn convert_to_pig_latin(word:&str)->String{
    let vowels=['a','e','i','o','u'];
    let mut chars:Vec<char>=word.chars().collect();//把每一个字符收集到一个向量中
    
    if let Some(&first_char)=chars.first(){ 
        /*Some(&first_char) 模式说明：

        chars.first() 返回 Some(&c) → 匹配成功

        &first_char → 解引用得到 char 类型（不是引用）

        如果匹配成功 → first_char 就是第一个字符，可以在大括号内使用

        如果匹配失败（即向量为空） → 条件不成立，跳过大括号内的代码 */
        if vowels.contains(&first_char.to_ascii_lowercase()){//把字符转化为ASCII小写
            //如果是元音开头，添加"- hay"
            format!("{}-hay",word)
        }
        else{
            //如果是辅音开头，将第一个字符移到末尾并并添加"ay"
            chars.remove(0);
            let rest:String=chars.into_iter().collect();
            format!("{}-{}ay",rest,first_char)
        }
    }
    else {
        word.to_string()
    }
}

fn employee_management(){
    println!("\n{}", "练习3：员工管理系统".blue().bold());

    let mut company=HashMap::new();

    //添加员工到部门
    add_employee_to_department(&mut company, "Engineering", "Alice");
    add_employee_to_department(&mut company, "Engineering", "Bob");
    add_employee_to_department(&mut company, "Sales", "Charlie");
    add_employee_to_department(&mut company, "Sales", "David");
    add_employee_to_department(&mut company, "Engineering", "Eve");

    //显示所有部门和员工
    print_company_structure(&company);

    //HashMap<String,Vec<String>>
    //获取特定部门的员工
    if let Some(engineers)=company.get("Engineering"){
        println!("\nEngineering部门的员工:");
        for employee in engineers{ 
            println!(" {}",employee);
        }
    }
}

fn add_employee_to_department(company:&mut HashMap<String,Vec<String>>,department:&str,employee:&str){
    company
        .entry(department.to_string())
        .or_insert_with(Vec::new)
        .push(employee.to_string());//因为Vec<String>
}

fn print_company_structure(company:&HashMap<String,Vec<String>>){
    println!("\n公司结构:");
    for (department,employees) in company{
        println!("{}部门:", department);
        let mut sorted_employees=employees.clone();
        sorted_employees.sort();
        for employee in sorted_employees{
            println!("  {}", employee);
        }
    }
}
fn main()
{
    run();
}