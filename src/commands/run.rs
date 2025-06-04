use crate::core::executor::CommandExecutor;
use std::char;
use std::io::{self, Read, Write};

pub fn handle_command_execution(command_parts: &str, y: bool) {
    if !y {
        print!("Are you sure you want to execute this :\" {command_parts} \" ");
        io::stdout().flush().unwrap();
        let ch = Read::bytes(io::stdin()).next().unwrap().unwrap() as char;

        if ch == 'N' || ch == 'n' {
            println!("Exiting...");
            return;
        }
    }
    let _ = match CommandExecutor::execute_at_once(command_parts) {
        Ok(o) => {
            // println!("stdout: \n{}", o.output_str);
            println!("status code: {}", o.status_code);
            o.output_str
        }
        Err(err) => {
            eprintln!(
                "\x1b[31mCommand failed with exit code: {}\x1b[0m",
                err.status_code
            );
            err.output_str
        }
    };
}
