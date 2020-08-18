use std::env;
use std::process;
mod parser;
mod codewriter;

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
    let mut newfile = codewriter::Codewriter::new(filename_without_extension)
                    .expect("erro");

    for a in file.content.lines(){
        let cm = match file.get_command(a) {
            Ok(n) => n,
            Err(e) => {
                println!("{}",e);
                process::exit(1);
            },
        };
        newfile.write(&cm);
        cm.print();
    }

}
