use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn it_works(){
        let num = 4;
        assert_eq!(num,4);
    }
    #[test]
    fn search_test(){
        let query = "duct";
        let content = "\n
Rust is a all round package,
it is fast, productive, safe,
There's three !
        ";
        assert!(search(&query,&content),vec!["it is fast, productive, safe,"]);
    }
}
pub struct Config{
    pub query: String,
    pub filename : String,
}
impl Config{
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>{
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not put query string."),
        };
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't insert filename."),
        };
        Ok(Config{
            query,
            filename,
        })
    }
}
pub fn run(config:&Config)->Result<(),Box<dyn Error>>{
    let mut file = File::open(&config.filename)?;
    let mut content = String::new();
    file.read_to_string(& mut content)?;
    for line in search(&config.query,&content){
        println!("{:?}",line);
    }
    Ok(())
}

pub fn search<'a>(query:& str, content:&'a str)->Vec<&'a str>{
    content.lines()
                  .filter(|line| line.contains(query))
                  .collect()
}