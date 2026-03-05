use std::error::Error;
use std::path::StripPrefixError;
use std::{fs, result};
use std::env;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool, //是否大小写敏感
}

impl Config{
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>{
        args.next(); //env::args()返回的第一个值是程序名称

        let query=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string"), //加了return会提前结束函数
        };
        let filename=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a filename"),
        };

        let case_sensitive=env::var("CASE_SENSITIVE").is_err();
        Ok(Config{query,filename,case_sensitive})
    }
}

 pub fn run(config:Config)->Result<(),Box<dyn Error>>
{   
    let contents=fs::read_to_string(config.filename)?;
    //println!("{:?}",search(&config.query, &contents));

    let results=if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_intensitive(&config.query, &contents)
    };

    for line in results{ //遍历vec
        println!("{}",line);
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn case_sensitive(){
        let query="duct";
        let contents="Rust: 
safe, fast, productive.
Duct tape. ";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){ //大小写不敏感
        let query="rUsT";
        let contents="Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(vec!["Rust:","Trust me."],search_case_intensitive(query, contents));
    }
    
}

pub fn search_case_intensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let query =query.to_lowercase(); //转化成小写
    let mut results=Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    contents.lines() //lines按换行符分开
        .filter(|line| line.contains(query)) //filter过滤如果包含true则进行下一步
        .collect()
}