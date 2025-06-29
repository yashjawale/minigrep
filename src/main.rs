use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args);

	let content = fs::read_to_string(&config.file_path).expect("Unable to read the file");

	println!("Searching for {}", config.query);
	println!("In the file {}", config.file_path);
	println!("With the content {}", content);
}

struct Config {
	query: String,
	file_path: String
}

impl Config {
	fn new(args: &[String]) -> Config {
		let query = args[1].clone();
		let file_path = args[2].clone();

		Config { query, file_path }
	}
}