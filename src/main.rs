use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searhing for {}", query);

    println!( "In file {}", filename);

    let contents = fs::read_to_string(filename)
       .expect("File not found");
    println!("With text:\n{}", contents);

}
