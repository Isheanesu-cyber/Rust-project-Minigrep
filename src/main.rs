extern crate minigrep;
use std::env;
use std::process;

fn main(){
	let args:Vec<String> = env::args().collect();
	let config = minigrep::Config::new(&args).unwrap_or_else( |err| {
		eprintln!("Problem parsing arguments :{:?}",err );
		process::exit(1);
	});

	println!("Searching for : {:?}",config.query);
	println!("File in :{:?}",config.filename );

	if let Err(error) = minigrep::run(config){
		eprintln!("Application problem :{:?}",error);
		process::exit(1);
	};

}