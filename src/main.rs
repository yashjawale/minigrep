use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

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
	fn build(args: &[String]) -> Result<Config, &str> {

		if args.len() < 3 {
			return Err("Not enough arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config { query, file_path })
	}
}