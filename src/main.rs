//to make the program to read the terminal arguments, we use std::env::args, it returns the iterator to the arguments passed in terminal
use std::env;
use std::fs; //this library helps to access and handle the filesystem

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments !");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect(); //Here collect function is reading and storing the arguments in the vector called args, it along with arguments stores the binary of the program which can be helpful to use name of program in any application.
    let config = Config::new(&args);
    //read and store the content of the file of the provided file path
    let content = fs::read_to_string(config.file_path).expect("Failed to read file!");
    dbg!(content); //returns the content of the file
}
