use std::path::PathBuf;

use clap::{builder::PathBufValueParser, Arg, Command};

fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("A cool file")
                .value_parser(PathBufValueParser::default()),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let default_file = PathBuf::from("input.txt");
    let myfile = matches.get_one("file").unwrap_or(&default_file);
    println!("The file passed is: {}", myfile.display());

    let num_str: Option<&String> = matches.get_one("num");
    match num_str {
        None => println!("No number"),
        Some(s) => {
            let parsed = s.parse::<i32>();
            match parsed {
                Ok(n) => println!("Favorite number is {}", n + 5),
                Err(_) => println!("`{}` is not a number!", s),
            }
        }
    }
}
