mod todo_list;
use crate::todo_list::ToDoList;
use std::io::{stdin, stdout, Write};
use fancy::{printcol, printcoln};

fn command_handler(todo_list: &mut ToDoList, cmd: Vec<&str>) {
    match cmd[0] {
        "add" => {
            let t = todo_list.add_cui(cmd);
            printcoln!("[green|b]Add[:] [bold]'{}'[:] to your list", t.unwrap());
            printcoln!("[cyan|b]Your updated todos:");
            todo_list.print_list()
        },
        "remove" => {
            let i = todo_list.remove_cui(&cmd);
            if i.is_some() {
                printcoln!("[red|b]Remove[:] task [bold]'{}'[:] from your todo list", i.unwrap().to_string());
                if !todo_list.is_empty() {
                    printcoln!("[cyan|b]Your updated todos:");
                }
                todo_list.print_list();
            }

        },
        "list" => {todo_list.print_list()}
        _ => println!("Command not found"),
    }
}

fn main() {
    let mut todo_list = ToDoList::new();
    loop {
        printcol!("[green|b]todo-cmd>[:] ");
        stdout().flush().unwrap();

        let mut full_cmd = String::new();
        stdin().read_line(&mut full_cmd).unwrap();
        full_cmd = full_cmd.trim().to_string(); //remove leading/trailing whitespace
        if full_cmd.is_empty() { printcoln!("[red]Please enter a command"); continue; }
        let cmd = full_cmd.split_whitespace().collect::<Vec<&str>>();

        if cmd[0] == "exit" || cmd[0] == "quit" {
            printcoln!("[b]Quitting...");
            break;
        }
        else {
            command_handler(&mut todo_list, cmd);
        }
    }
}
