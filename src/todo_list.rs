mod task;

use std::io::{stdin, stdout, Write};
use crate::todo_list::task::Task;
use fancy::{printcol, printcoln};

pub struct ToDoList {
    task_list: Vec<Task>
}

impl ToDoList {
    pub(crate) fn new() -> Self {
        Self { task_list: Vec::new() }
    }

    fn add(&mut self, task: Task){
        self.task_list.push(task);
    }

    pub fn add_cui(&mut self, cmd: Vec<&str>) -> Option<String>{
        let mut title = String::new();
        if cmd.len() == 1 {
            // Let user add task title
            printcol!("[magenta|b]Please input task title[:]: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut title).unwrap();
            title = title.trim().parse().unwrap();
        }
        else {
            title = cmd[1..].join(" ").trim().to_string();
        }
        // Create Task object
        let task = Task::new(title.clone());
        self.add(task);
        Some(title)
    }

    pub fn remove(&mut self, index: i32) -> bool{
        if self.task_list.is_empty() || index as usize > self.task_list.len() || index < 0
        {
            printcoln!("[red]Please input valid task index[:]. \
            You can check the list using command `[b]list[:]`.");
            return false;
        }
        self.task_list.remove(index as usize);
        true
    }

    pub(crate) fn remove_cui(&mut self, cmd: &Vec<&str>) -> Option<i32>{
        let mut task_number = String::new();
        if cmd.len() == 1 {
            printcol!("[magenta|b]Please input task number:[:] ");
            stdout().flush().unwrap();
            stdin().read_line(&mut task_number).unwrap();
            task_number = task_number.trim().parse().unwrap();
        }
        else {
            task_number = cmd[1].to_string();
        }
        match task_number.trim().parse::<i32>() {
            Ok(i) => {
                if self.remove(i-1){
                    Some(i)
                }
                else {
                    None
                }
            },
            Err(_) => {
                printcoln!("[red|b]invalid input[:]. remove use task number as input.");
                None
            }
        }
    }

    pub fn print_list(&self) {
        if self.task_list.is_empty() {
            println!("Your todo list is empty.");
        }
        self.task_list.iter().enumerate()
            .for_each(|(i, t)| {println!("[{}] {}", i + 1, t.to_string())})
    }

    pub fn is_empty(&self) -> bool{
        self.task_list.is_empty()
    }
}