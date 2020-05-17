/**
 * Main Driver class for shell
 * Author: Colby Allen
 **/ 
 use std::io::prelude::*;                                                        
 use std::io;

fn main() {
    
    loop {
        print!(">> ");
        io::stdout().flush().ok().expect("Could not flush stdout");    

        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");

        print!("{}", command);
    }
}

