fn old_main() {
// read line from std input
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
    }
}