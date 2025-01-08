use std::io::{stdin, stdout, Write};

fn command_handler(cmd: Vec<&str>) {
    match cmd[0] {
        "add" => println!("Added {}", cmd[1].to_string()),
        "remove" => println!("Remove {}", cmd[1].to_string()),
        _ => println!("Command not found"),
    }
}

fn main() {
    loop {
        print!("todo-cmd> ");
        stdout().flush().unwrap();

        let mut full_cmd = String::new();
        stdin().read_line(&mut full_cmd).unwrap();
        full_cmd = full_cmd.trim().to_string(); //remove leading/trailing whitespace
        let cmd = full_cmd.split_whitespace().collect::<Vec<&str>>();

        if cmd[0] == "exit" || cmd[0] == "quit" {
            println!("Quitting...");
            break;
        }
        else {
            command_handler(cmd);
        }
    }
}
