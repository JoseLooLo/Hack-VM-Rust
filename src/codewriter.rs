use std::fs;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

pub struct Codewriter {
    pub file: fs::File,
}

impl Codewriter {
    pub fn new(filename: &str) -> Result<Codewriter, io::Error> {
        match OpenOptions::new().append(true).create(true).open(filename) {
            Ok(n) => Ok(Codewriter{
                file: n,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn write(&mut self, command: &vm::Command) {
        match &command.command_type {
            &vm::Type::Arithmetic => {
                self.write_arithmetic(command);
            },
            &vm::Type::Push => {
                self.write_push(command);
            },
            &vm::Type::Pop => {
                self.write_pop(command);
            },
            _ => {},
        }
        let arg1 = &command.arg1;
        self.file.write(arg1.as_bytes()).expect("a");
    }

    fn write_arithmetic(&mut self, _command: &vm::Command) {
        //
    }

    fn write_push(&mut self, _command: &vm::Command) {
        //get constant
        let constant = _command.arg2.to_string();
        let constant = format!("@{}",constant);
        
    }

    fn write_pop(&mut self, _command: &vm::Command) {
        //
    }

    fn write_increment_stack(&mut self) {
        //
    }

    fn write_decrement_stack(&mut self) {
        //
    }
}