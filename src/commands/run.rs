use crate::core::executor::CommandExecutor;
use std::char;
use std::io::{self, Read, Write};

pub async fn handle_command_execution(command_parts: &str, y: bool, v: bool) {
    if !y {
        print!(
            "are you sure you want to execute this :\" {} \" ",
            command_parts
        );
        io::stdout().flush().unwrap();
        let ch = io::stdin().bytes().next().unwrap().unwrap() as char;

        if ch == 'N' || ch == 'n' {
            println!("nice you don't");
            return;
        }
    }
    match CommandExecutor::execute(command_parts, v) {
        Ok((stdout_str, status_i32)) => {
            // println!("stdout: \n{}", stdout_str);
            println!("status code: {}", status_i32);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
