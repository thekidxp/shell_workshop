use std::{io, env};
use std::process::Command;
use std::io::Write;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let commands = input.split(';');

        for command in commands {
            let binary: String = command.split_whitespace().take(1).collect();
            let args: Vec<_> = command.split_whitespace().skip(1).collect();

            if binary.eq_ignore_ascii_case("cd") {
                change_dir(&args[0]);
                println!("{}", env::current_dir().unwrap().display());

            } else {
                let output = Command::new(&binary)
                    .args(&args)
                    .output()
                    .expect("failed to execute process");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}

fn change_dir(path: &str){
    env::set_current_dir(path).expect("problem");
}
