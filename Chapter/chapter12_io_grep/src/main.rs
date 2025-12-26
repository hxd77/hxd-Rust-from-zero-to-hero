use std::env;
use std::process;

use chapter12_io_grep::Config;

fn main() {
    let args:Vec<String>=env::args().collect();


    let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err); //将Err("not enough arguments")中的not enough arguments传递给err
    process::exit(1);
    }); // 注意这里圆括号在最后
    
    //println!("Searching for {}",config.query);
    //println!("In file {}",config.filename);

    if let Err(e)=chapter12_io_grep::run(config){
        println!("Appllication error: {}",e);

        process::exit(1);
    }
}

