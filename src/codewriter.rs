use std::fs;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

pub struct Codewriter {
    pub file: fs::File,
    filename: String,
    counter: i32,
}

impl Codewriter {
    pub fn new(filename: &str) -> Result<Codewriter, io::Error> {
        match OpenOptions::new().append(false).write(true).create(true).open(filename) {
            Ok(n) => Ok(Codewriter{
                file: n,
                filename: filename.split('.').collect::<Vec<&str>>()[0].to_string().to_uppercase(),
                counter: 0,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn write(&mut self, command: &crate::Command) {
        match &command.command_type {
            &crate::Type::Arithmetic => {
                self.write_arithmetic(command);
            },
            &crate::Type::Push => {
                self.write_push(command);
            },
            &crate::Type::Pop => {
                self.write_pop(command);
            },
            _ => {},
        }
    }

    fn write_arithmetic(&mut self, _command: &crate::Command) {
        if _command.arg1 == "add".to_string() {
            self.file.write(self.get_add().as_bytes()).expect("erro add");
        } else if _command.arg1 == "sub".to_string() {
            self.file.write(self.get_sub().as_bytes()).expect("erro sub");
        } else if _command.arg1 == "neg".to_string() {
            self.file.write(self.get_neg().as_bytes()).expect("erro neg");
            self.counter += 1;
        } else if _command.arg1 == "eq".to_string() {
            self.file.write(self.get_eq().as_bytes()).expect("erro eq");
            self.counter += 1;
        } else if _command.arg1 == "gt".to_string() {
            self.file.write(self.get_gt().as_bytes()).expect("erro gt");
            self.counter += 1;
        } else if _command.arg1 == "lt".to_string() {
            self.file.write(self.get_lt().as_bytes()).expect("erro lt");
            self.counter += 1;
        } else if _command.arg1 == "not".to_string() {
            self.file.write(self.get_not().as_bytes()).expect("erro not");
            self.counter += 1;
        } else if _command.arg1 == "and".to_string() {
            self.file.write(self.get_and().as_bytes()).expect("erro and");
            self.counter += 1;
        } else if _command.arg1 == "or".to_string() {
            self.file.write(self.get_or().as_bytes()).expect("erro or");
            self.counter += 1;
        } 
    }

    fn get_neg(&self) -> String {
        let mut cmd = String::from("//neg\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += "D=-D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_or(&self) -> String {
        let mut cmd = String::from("//and\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M|D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_and(&self) -> String {
        let mut cmd = String::from("//and\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M&D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd

    }

    fn get_not(&self) -> String {
        let mut cmd = String::from("//not\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += "D=!D\n";
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_eq(&self) -> String {
        let mut cmd = String::from("//eq\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "A=M\n";
        cmd += "D=A-D\n";
        cmd += &self.get_equals_label();
        cmd += "D;JEQ\n";
        //Not equals
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += &self.get_end_label();
        cmd += "D;JMP\n";
        cmd += &self.get_equals_label_jump();
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += "D=D-1\n";
        cmd += &self.get_end_label_jump();
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_gt(&self) -> String {
        let mut cmd = String::from("//gt\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "A=M\n";
        cmd += "D=A-D\n";
        cmd += &self.get_gt_label();
        cmd += "D;JGT\n";
        //Not greater than
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += &self.get_end_label();
        cmd += "D;JMP\n";
        cmd += &self.get_gt_label_jump();
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += "D=D-1\n";
        cmd += &self.get_end_label_jump();
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
    }

    fn get_lt(&self) -> String {
        let mut cmd = String::from("//lt\n");
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "D=M\n";
        cmd += &self.get_decrement_stack();
        cmd += "A=M\n";
        cmd += "A=M\n";
        cmd += "D=A-D\n";
        cmd += &self.get_lt_label();
        cmd += "D;JLT\n";
        //Not greater than
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += &self.get_end_label();
        cmd += "D;JMP\n";
        cmd += &self.get_lt_label_jump();
        cmd += &self.get_constant_string(0);
        cmd += "D=A\n";
        cmd += "D=D-1\n";
        cmd += &self.get_end_label_jump();
        cmd += "@SP\n";
        cmd += "A=M\n";
        cmd += "M=D\n";
        cmd += &self.get_increment_stack();

        cmd
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
        let mut cmd = String::from("//sub\n");
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

    fn write_push(&mut self, _command: &crate::Command) {
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
        } else if _command.arg1 == "pointer".to_string() {
            if _command.arg2 == 0 {
                cmd += &self.get_segment_string(&"this".to_string());
                cmd += "D=M\n";
                cmd += "@SP\n";
                cmd += "A=M\n";
                cmd += "M=D\n";
                cmd += &self.get_increment_stack();
            } else {
                cmd += &self.get_segment_string(&"that".to_string());
                cmd += "D=M\n";
                cmd += "@SP\n";
                cmd += "A=M\n";
                cmd += "M=D\n";
                cmd += &self.get_increment_stack();
            }

            self.file.write(cmd.as_bytes()).expect("erro push pointer");
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

    fn write_pop(&mut self, _command: &crate::Command) {
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
        } else if _command.arg1 == "pointer".to_string() {
            if _command.arg2 == 0 {
                cmd += &self.get_decrement_stack();
                cmd += "@SP\n";
                cmd += "A=M\n";
                cmd += "D=M\n";
                cmd += &self.get_segment_string(&"this".to_string());
                cmd += "M=D\n";
                //cmd += "M=D\n";
            } else {
                cmd += &self.get_decrement_stack();
                cmd += "@SP\n";
                cmd += "A=M\n";
                cmd += "D=M\n";
                cmd += &self.get_segment_string(&"that".to_string());
                //cmd += "A=M\n";
                cmd += "M=D\n";
            }

            self.file.write(cmd.as_bytes()).expect("erro pop pointer");
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

    fn get_equals_label_jump(&self) -> String {
        format!("(EQ{})\n", self.counter.to_string())
    }

    fn get_equals_label(&self) -> String {
        format!("@EQ{}\n", self.counter.to_string())
    }

    fn get_end_label(&self) -> String {
        format!("@END{}\n", self.counter.to_string())
    }

    fn get_end_label_jump(&self) -> String {
        format!("(END{})\n", self.counter.to_string())
    }

    fn get_gt_label(&self) -> String {
        format!("@GT{}\n", self.counter.to_string())
    }

    fn get_gt_label_jump(&self) -> String {
        format!("(GT{})\n", self.counter.to_string())
    }

    fn get_lt_label(&self) -> String {
        format!("@LT{}\n", self.counter.to_string())
    }

    fn get_lt_label_jump(&self) -> String {
        format!("(LT{})\n", self.counter.to_string())
    }

}