use std::fs;
use std::io;

pub struct File {
    pub content: String,
}

impl File {
    pub fn new(filename: &String) -> Result<File, io::Error> {
        let open_file = fs::read_to_string(filename);
        match open_file {
            Ok(f) => Ok(File{
                content: f,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_command(&self, _command: &str) -> Result<vm::Command, String> {
        let mut comm = vm::Command::new();
        let c = _command.trim()
                .to_string()
                .to_lowercase();
        let c: Vec<&str> = c.split_whitespace()
                            .collect();

        comm.command_type = self.get_type(c[0])?;
        if c.len() == 1 {
            comm.arg1 = self.get_arg1(&comm.command_type, c[0])?;
        } else if c.len() == 2 {
            comm.arg1 = self.get_arg1(&comm.command_type, c[1])?;
        } else if c.len() == 3 {
            comm.arg1 = self.get_arg1(&comm.command_type, c[1])?;
            comm.arg2 = self.get_arg2(&comm.command_type, c[2])?;
        } else {
            return Err("Invalid command".to_string());
        }
        return Ok(comm);
    }

    fn get_type(&self, _type: &str) -> Result<vm::Type, String> {
        if _type == "add" || _type == "sub" || _type == "neg" {
            return Ok(vm::Type::Arithmetic)
        } else if _type == "eq" || _type == "gt" || _type == "lt" {
            return Ok(vm::Type::Arithmetic)
        } else if _type == "and" || _type == "or" || _type == "not" {
            return Ok(vm::Type::Arithmetic)
        } else if _type == "push" {
            return Ok(vm::Type::Push)
        } else if _type == "pop" {
            return Ok(vm::Type::Pop)
        } else {
            return Err("Invalid command".to_string())
        }
    }

    fn get_arg1(&self, _type: &vm::Type, _arg1: &str) -> Result<String, String> {
        match _type {
            &vm::Type::Arithmetic => {
                return Ok(_arg1.to_string());
            },
            &vm::Type::Push => {
                return Ok(_arg1.to_string());
            },
            &vm::Type::Pop => {
                return Ok(_arg1.to_string());
            },
            _ => {
                return Err("Invalid command".to_string());
            },
        }
    }

    fn get_arg2(&self, _type: &vm::Type, _arg2: &str) -> Result<i32, String> {
        let _arg2 = match _arg2.parse::<i32>(){
            Ok(n) => n,
            Err(_e) => return Err("Invalid command".to_string()),
        };
        match _type {
            &vm::Type::Push => {
                Ok(_arg2)
            },
            &vm::Type::Pop => {
                Ok(_arg2)
            },
            _ => {
                return Err("Invalid command".to_string());
            },
        }
    }
}