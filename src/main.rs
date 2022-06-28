use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let matches = App::new("generate_password")
        .version("0.1.2")
        .author("Ivan Terekhin <i.terhin@gmail.com>")
        .about(
            "Use this package to generate password or any 'random' strings with length you want.",
        )
        .arg(
            Arg::with_name("length")
                .value_name("LENGTH")
                .help("Length of generated password")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let len = matches.value_of("length").unwrap();
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

fn help() -> () {
    println!("Provide an argument with length of the password");
}
