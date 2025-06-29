use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = parse_config(&args);

	let content = fs::read_to_string(&config.filepath).expect("Unable to read the file");

	println!("Searching for {}", config.query);
	println!("In the file {}", config.filepath);
	println!("With the content {}", content);
}

struct Config {
	query: String,
	filepath: String
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filepath = args[2].clone();

	Config { query, filepath }
}