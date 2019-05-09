use std::{env, process};

use starshine::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match Command::new(&args) {
        Ok(c) => c,
        Err(e) => {
            exit_with_error(e)            
        }
    };

    match starshine::run(&command) {
        Ok(_) => process::exit(0),
        Err(e) => {
            exit_with_error(e)
        }
    }
}

fn exit_with_error(err: &str) -> ! {
    println!("error: {}", err);
    process::exit(0);
}


