use slay_grep::Config;
use std::env; //to make the program to read the terminal arguments, we use std::env::args, it returns the iterator to the arguments passed in terminal
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); //Here collect function is reading and storing the arguments in the vector called args, it along with arguments stores the binary of the program which can be helpful to use name of program in any application.
    let config = Config::build(&args).unwrap_or_else(|e| {
        //unwrap_or_else help us to define some non panic custom error handling it is very similar to unwrap function
        eprintln!("Error occured while building config: {e}",);
        process::exit(1);
    });
    if let Err(e) = slay_grep::run(config) {
        eprintln!("Error occured while running config: {e}");
        process::exit(1);
    }
}
