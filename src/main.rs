use std::env;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let len = &args[1];
            let length: usize = match len.parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("error: argument not an integer");
                    help();
                    return;
                }
            };
            let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect();

            println!("{}", rand_string);
        }
        _ => {
            help();
        }
    }
}

fn help() -> () {
    println!("Provide an argument with length of the password");
}
