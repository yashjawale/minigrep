use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	println!("Searching for {}", config.query);
	println!("In the file {}", config.file_path);

	if let Err(e) = minigrep::run(config) {
		println!("Application error: {e}");
		process::exit(1);
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	vec![]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive"], search(query, contents));
	}
}