use std::env;
use std::str::FromStr;

mod constant;
mod text;

use text::dispatch_logic::portal;
use constant::DispatchLogicType;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // one argument passed
        2 => {
            println!("{:?}",args[1]);
            let arg = DispatchLogicType::from_str(&args[1]).unwrap();

            println!("{:?}",portal::text(&arg));
            match arg {
                // Ok(42) => println!("This is the answer!"),
                DispatchLogicType::Normal => println!("This is ! normal"),
                DispatchLogicType::Seller => println!("This is ! seller"),
                // _ => println!("This is not the answer."), // unreachable
            }
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
