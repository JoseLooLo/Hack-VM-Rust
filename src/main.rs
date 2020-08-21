use std::env;
use std::process;
mod parser;
mod codewriter;

#[derive(Debug)]
pub enum Type {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub struct Command {
    pub command_type: Type,
    pub arg1: String,
    pub arg2: i32,
}

impl Command {
    pub fn new() -> Command {
        Command{
            command_type: Type::Arithmetic,
            arg1: String::new(),
            arg2: -1,
        }
    }

    pub fn print(&self) {
        println!("Command Type -> {:?}",self.command_type);
        println!("Arg1 -> {}", self.arg1);
        println!("Arg2 -> {}", self.arg2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument. Got {}.", (args.len() - 1));
        process::exit(1);
    }

    let filename = &args[1];
    let file = parser::File::new(filename)
                .expect("File error");

    let filename_without_extension = filename.split('.')
                                    .collect::<Vec<&str>>()[0];
    let filename_with_extension = format!("{}.{}", filename_without_extension, "asm");
    let mut newfile = codewriter::Codewriter::new(&filename_with_extension)
                    .expect("erro");

    for a in file.content.lines(){
        let _ = match file.get_command(a) {
            Ok(n) => {
                newfile.write(&n);
                n.print();
            },
            Err(e) => {
                println!("{}",e);
                //process::exit(1);
            },
        };
    }

}