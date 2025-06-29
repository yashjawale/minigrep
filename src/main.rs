use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let query = &args[1];
	let filepath = &args[2];

	let content = fs::read_to_string(filepath).expect("Unable to read the file");

	println!("Searching for {}", query);
	println!("In the file {}", filepath);
	println!("With the content {}", content);
}
