use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn search_test(){
		let query = "duct";
		let content = "\
fast,safe,productive,
That's three for you!";
		assert_eq!(Ok(vec!["fast,safe,productive,"]),search(&query,&content));
	}
}

pub struct Config{
	pub query :String,
	pub filename :String,
}
impl Config{
	pub fn new(args:&[String])->Result<Config,&'static str>{
		if args.len() < 3{
			return Err("not enough arguments !")
		}
		let query = args[1].clone();
		let filename = args[2].clone();
		Ok(Config{query, filename,})
	}
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
	let mut file = File::open(config.filename)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	for line in search(&config.query,&content)?{
		println!("{:?}",line );
	}
	Ok(())
}

pub fn search<'a>(query:&str,content:&'a str)->Result<Vec<&'a str>,&'a str>{
	let mut result = Vec::new();
	for line in content.lines(){
		if line.contains(query){
			result.push(line);
		}
	}
	Ok(result)
}




