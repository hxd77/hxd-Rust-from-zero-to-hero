use core::panic;
use std::env;
use std::env::Args;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args:Vec<String>=env::args().collect();


    let config = Config::new(&args).unwrap_or_else(|err|){
        println!("Problem parsing arguments: {}",err); //将Err("not enough arguments")中的not enough arguments传递给err
        process::exit(1);
    }

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e)=run(config){
        println!("Appllication error: [}",e);

        process:exit(1);
    }
}

fn run(config:Config)->Result<(),Box<dyn Error>>
{
    let contents=fs::read_to_string(config.filename)?;

    println!("With text:\n{}",contents);
    Ok(())
}
struct Config{
    query:String,
    filename:String,
}

impl Config{
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len() <3{
            return Err("not enough arguments");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
       

       Ok(Config{query,filename})
    }
}