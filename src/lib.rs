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