use std::fs;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

pub struct Codewriter {
    pub file: fs::File,
    filename: String,
}

impl Codewriter {
    pub fn new(filename: &str) -> Result<Codewriter, io::Error> {
        match OpenOptions::new().append(true).open(filename) {
            Ok(n) => Ok(Codewriter{
                file: n,
                filename: filename.split('.').collect::<Vec<&str>>()[0].to_string().to_uppercase(),
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
    }

    fn write_arithmetic(&mut self, _command: &vm::Command) {
        if _command.arg1 == "add".to_string() {
            self.file.write(self.get_add().as_bytes()).expect("erro add");
        } else if _command.arg1 == "sub".to_string() {
            self.file.write(self.get_sub().as_bytes()).expect("erro sub");
        }
    }

    fn get_add(&self) -> String {
        let mut cmd = String::from("//add\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "A=M\n";
        cmd += "D=A+D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_sub(&self) -> String {
        let mut cmd = String::from("//add\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "A=M\n";
        cmd += "D=A-D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn write_push(&mut self, _command: &vm::Command) {
        let mut cmd = format!("//push {} {}\n",_command.arg1, _command.arg2.to_string());
        if _command.arg1 == "constant".to_string() {
            cmd += &self.get_constant_string(_command.arg2); //@n
            cmd += "D=A\n";
            cmd += "@SP\n";
            cmd += "A=M\n";
            cmd += "M=D\n";
            cmd += &self.get_increment_stack();

            self.file.write(cmd.as_bytes()).expect("erro push constant");
        } else if _command.arg1 == "temp".to_string() {
            cmd += &self.get_temp_adress(_command.arg2);
            cmd += "D=M\n";
            cmd += "@SP\n";
            cmd += "A=M\n";
            cmd += "M=D\n";
            cmd += &self.get_increment_stack();

            self.file.write(cmd.as_bytes()).expect("erro push temp");

        } else if _command.arg1 == "static".to_string() {
            cmd += &self.get_static_name(_command.arg2);
            cmd += "D=M\n";
            cmd += "@SP\n";
            cmd += "A=M\n";
            cmd += "M=D\n";
            cmd += &self.get_increment_stack();

            self.file.write(cmd.as_bytes()).expect("erro push static");
        } else {
            cmd += &self.get_constant_string(_command.arg2); //@n
            cmd += "D=A\n";
            cmd += &self.get_segment_string(&_command.arg1);
            cmd += "A=M\n";
            cmd += "A=D+A\n";
            cmd += "D=M\n";
            cmd += "@SP\n";
            cmd += "A=M\n";
            cmd += "M=D\n";
            cmd += &self.get_increment_stack();

            self.file.write(cmd.as_bytes()).expect("erro push else");
        }
    }

    fn write_pop(&mut self, _command: &vm::Command) {
        let mut cmd = format!("//pop {} {}\n",_command.arg1, _command.arg2.to_string());
        if _command.arg1 == "temp".to_string() {
            cmd += &self.get_decrement_stack();
            cmd += "A=M\n";
            cmd += "D=M\n";
            cmd += &self.get_temp_adress(_command.arg2);
            cmd += "M=D\n";

            self.file.write(cmd.as_bytes()).expect("erro pop temp");
        } else if _command.arg1 == "static".to_string() {
            cmd += &self.get_decrement_stack();
            cmd += "A=M\n";
            cmd += "D=M\n";
            cmd += &self.get_static_name(_command.arg2);
            cmd += "M=D\n";

            self.file.write(cmd.as_bytes()).expect("erro pop static");
        } else {
            cmd += &self.get_constant_string(_command.arg2);
            cmd += "D=A\n";
            cmd += &self.get_segment_string(&_command.arg1);
            cmd += "A=M\n";
            cmd += "D=A+D\n";
            cmd += &self.get_temp_adress(0);
            cmd += "M=D\n";
            cmd += &self.get_decrement_stack();
            cmd += "A=M\n";
            cmd += "D=M\n";
            cmd += &self.get_temp_adress(0);
            cmd += "A=M\n";
            cmd += "M=D\n";

            self.file.write(cmd.as_bytes()).expect("erro pop else");
        }
    }

    fn get_increment_stack(&self) -> String {
        String::from("@SP\nM=M+1\n")
    }

    fn get_decrement_stack(&self) -> String {
        String::from("@SP\nM=M-1\n")
    }

    fn get_segment_string(&self, _arg: &String) -> String {
        if _arg == &"local".to_string() {
            return String::from("@LCL\n");
        } else if _arg == &"argument".to_string() {
            return String::from("@ARG\n");
        } else if _arg == &"this".to_string() {
            return String::from("@THIS\n");
        } else if _arg == &"that".to_string() {
            return String::from("@THAT\n");
        } else {
            return String::from("\n");
        }
    }

    fn get_constant_string(&self, _arg: i32) -> String {
        format!("@{}\n",_arg.to_string())
    }

    fn get_temp_adress(&self, _offset: i32) -> String {
        let a = 5 + _offset;
        format!("@{}\n", a.to_string())
    }

    fn get_static_name(&self, _offset: i32) -> String {
        format!("@{}.{}\n",self.filename, _offset.to_string())
    }
}