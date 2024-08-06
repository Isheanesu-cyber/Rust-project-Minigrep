extern crate grep;
use std::env;
use std::process;

fn main(){
    let config = grep::Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("Error parsing arguments : {:?}",err);
        process::exit(1);
    });
    println!("Searching for : {:?}",config.query);
    println!("In filename : {:?}",config.filename);
    if let Err(e) = grep::run(&config){
        eprintln!("Application error : {:?}",e);
        process::exit(1);
    }
}
