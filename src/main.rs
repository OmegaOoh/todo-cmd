mod todo_list;
use crate::todo_list::ToDoList;
use std::io::{stdin, stdout, Write};
use clearscreen::clear;
use fancy::{printcol, printcoln};

fn command_handler(todo_list: &mut ToDoList, cmd: Vec<&str>) {
    match cmd[0] {
        "clear" => {
            clear().unwrap();
        }
        "add" => {
            let t = todo_list.add_cui(cmd);
            printcoln!("[green|b]Add[:] [bold]'{}'[:] to your list", t.unwrap());
            printcoln!("[cyan|b]Your updated todos:");
            todo_list.print_list()
        },
        "remove" => {
            let i = todo_list.remove_cui(&cmd);
            if i.is_some() {
                printcoln!("[red|b]Remove[:] Todo Task #[bold]{}[:] from your todo list", i.unwrap().to_string());
                if !todo_list.is_empty() {
                    printcoln!("[cyan|b]Your updated todos:");
                }
                todo_list.print_list();
            }

        },
        "done" => {
            let i = todo_list.mark_as_done_cui(&cmd);
            if i.is_some() {
                printcoln!("[magenta|b]Marked task as Done.[:]");
            if !todo_list.is_empty() {
                printcoln!("[cyan|b]Your updated todos:");
            }
            todo_list.print_list();
        }},
        "undone" => {
            let i = todo_list.mark_undone_cui(&cmd);
            if i.is_some() {
                printcoln!("[yellow]Marked task as undone.");
            }
        }
        "clear_done" => {
            printcoln!("[yellow|b]This action cannot be undone. press y to confirm, anything else to cancel.");
            let mut confirmation = String::new();
            stdin().read_line(&mut confirmation).unwrap();
            if confirmation.to_lowercase().trim() == "y" {
                todo_list.clear_done();
                printcoln!("[magenta]Your Done tasks have been cleared.");
            } else {
                printcoln!("[blue]Operation canceled");
            }

        }
        "list" => {todo_list.print_list()},
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
