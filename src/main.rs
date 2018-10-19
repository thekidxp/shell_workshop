use std::io;
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

            let output = Command::new(&binary)
                .args(&args)
                .output()
                .expect("failed to execute process");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        /*        // read line from std input
                let mut input = String::new();

                io::stdin().read_line(&mut input).unwrap();

                // parse line into executable command
                let command: Vec<&str> = input.split(" ").collect();

                // execute the command in a separate process

                if command.len() == 1 {
                    let output =
                        Command::new(command[0])
                            .output()
                            .expect("failed to execute process");

                    // show output
                    let display = String::from_utf8(output.stdout).expect("something went wrong!");
                    println!("{}", display);
                } else if command.len() > 1 {
                    if let Some((command, arguments)) = command.split_first() {
                        let output =
                            Command::new(command)
                                .args(arguments)
                                .output()
                                .expect("failed to execute process");

                        // show output
                        let display = String::from_utf8(output.stdout).expect("something went wrong!");
                        println!("{}", display);
                    }
                }*/
    }
}
