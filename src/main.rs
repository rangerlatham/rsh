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

struct Cursor{
    x: u16,
    y: u16,
}

fn main() {
    //clear screen, set up terminal
    let prompt = String::from("write your input:");
    print!("{clear}{goto}",clear = clear::BeforeCursor, goto = cursor::Goto(1,1));
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor = Cursor{x:prompt.chars().count() as u16,y:1};

    loop{
        let mut input = String::new();
        write!(stdout,"{}",prompt).unwrap();
        stdout.flush().unwrap();

        for c in io::stdin().keys(){
            match c.unwrap(){
                Key::Char('\t') => print!("tab"),
                Key::Char('\n') =>{
                    cursor.x=prompt.chars().count() as u16;
                    break;
                },
                Key::Backspace => {
                    if cursor.x >=18{
                        cursor.x-=1;
                        input.pop();
                        print!("{goleft}{erase}",
                            goleft = cursor::Left(1), erase = clear::UntilNewline);
                    }
                },
                Key::Ctrl('l') => print!("{clear}{goto}{prompt}",
                    clear = clear::BeforeCursor, goto = cursor::Goto(1,1),prompt = prompt),
                Key::Up => print!("up"),
                Key::Esc => print!("esc"),
                Key::Char(c) => {
                    cursor.x+=1;
                    let c2: char = c;
                    print!("{}",c2);
                    input.push(c);
                },
                Key::Ctrl('c') => return,
                Key::Alt(c) => print!("Alt+{}",c),
                Key::Ctrl(c) => print!("Ctrl+{}",c),
                _ => print!("other"),
            }
            stdout.flush().unwrap();
        }

        print!("\n\r");
        stdout.flush().unwrap();

        let input: Vec<&str> = input.trim().split(" ").collect(); //TODO: ERROR HANDLING
        if execute(input[0],input[1..].to_vec()) == 1{
            break; //TODO: deal with errors/exit codes
        }
        print!("\r");
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
