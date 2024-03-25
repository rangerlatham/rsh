extern crate termion;

use termion::{clear, cursor};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{self, Write};
use std::env;
use std::process::{Command, Stdio};
use std::path::Path;
//use std::fs;

fn main() {
    print!("{clear}{goto}",clear = clear::BeforeCursor, goto = cursor::Goto(1,1));
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    loop{

        //let _ = io::stdout().write(b"write your input:");
        //let _ = io::stdout().flush();
        let mut input = String::new();
        write!(stdout,"write your input:").unwrap();
        stdout.flush().unwrap();
        for c in io::stdin().keys(){
            match c.unwrap(){
                Key::Char('\t') => print!("tab"),
                Key::Char('\n') => print!("return"),
                Key::Backspace => print!("backspace"),
                Key::Up => print!("up"),
                Key::Esc => print!("esc"),
                Key::Char(c) => print!("{}",c),
                Key::Ctrl('c') => return,
                Key::Alt(c) => print!("Alt+{}",c),
                Key::Ctrl(c) => print!("Ctrl+{}",c),
                _ => print!("other"),
            }
            stdout.flush().unwrap();
        }
        //io::stdin().read_line(&mut input).unwrap(); //TODO: ERROR HANDLING

        //let input: Vec<&str> = input.trim().split(" ").collect(); //TODO: ERROR HANDLING
        //if execute(input[0],input[1..].to_vec()) == 1{
            //break; //TODO: deal with errors/exit codes
        //}
    }
}

fn execute(command: &str, args: Vec<&str>) -> u8{ //TODO: return exit code of child process
    match command{
        "cd" => change_directory(args),
        "pwd" => print_directory(),
        "exit" => 1,
        _ => {
            let _cmd = Command::new(command) //TODO: ERROR HANDLING
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .stdin(Stdio::inherit())
                .output()
                .expect("Command failed to execute");
            0
            }

    }
}

fn print_directory() -> u8{
    println!("{}",env::current_dir().unwrap().display()); //TODO: ERROR HANDLING
    0
}

fn change_directory(args: Vec<&str>) -> u8{ //TODO: posix-defined functionality
    let _  =env::set_current_dir(Path::new(args[0])); //TODO: ERROR HANDLING
    0
}

//TODO: handle signals
//TODO: more obscure shell builtins
//TODO: prompts
//TODO: history
