use std::env;
mod todo;
use todo::{add, change, delete, list, print_help, Command};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let commad = Command::from(&args[1]);
        match commad {
            Ok(Command::Add) => {
                add(&args);
            },
            Ok(Command::Change) => {
                change(&args)
            },
            Ok(Command::Delete) => {
                delete(&args)
            },
            Ok(Command::Help) => {
                print_help();
            },
            Ok(Command::List) => {
                list();
            },
            _command_error => {
                println!("Invalid Command! Type 'cargo run help' for usage.");
                std::process::exit(1);
            }
        }
    } else {
        println!("Please input command");
        std::process::exit(1);
    }

}