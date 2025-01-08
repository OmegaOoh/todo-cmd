mod task;

use std::io::{stdin, stdout, Write};
use crate::todo_list::task::Task;
use fancy::{printcol, printcoln};

pub struct ToDoList {
    task_list: Vec<Task>,
    done_list: Vec<Task>,
}

impl ToDoList {
    pub(crate) fn new() -> Self {
        Self { task_list: Vec::new(), done_list: Vec::new() }
    }

    fn add(&mut self, task: Task) {
        self.task_list.push(task);
    }
    fn remove(&mut self, index: i32) -> bool {
        if self.task_list.is_empty() || index as usize > self.task_list.len() || index < 0
        {
            printcoln!("[red]Please input valid task index[:]. \
            You can check the list using command `[b]list[:]`.");
            return false;
        }
        self.task_list.remove(index as usize);
        true
    }

    pub fn add_cui(&mut self, cmd: Vec<&str>) -> Option<String> {
        let mut title = String::new();
        if cmd.len() == 1 {
            // Let user add task title
            printcol!("[magenta|b]Please input task title[:]: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut title).unwrap();
            title = title.trim().parse().unwrap();
        } else {
            title = cmd[1..].join(" ").trim().to_string();
        }
        // Create Task object
        let task = Task::new(title.clone());
        self.add(task);
        Some(title)
    }


    pub(crate) fn remove_cui(&mut self, cmd: &Vec<&str>) -> Option<i32> {
        let mut task_number = String::new();
        if cmd.len() == 1 {
            printcol!("[magenta|b]Please input task number:[:] ");
            stdout().flush().unwrap();
            stdin().read_line(&mut task_number).unwrap();
            task_number = task_number.trim().parse().unwrap();
        } else {
            task_number = cmd[1].to_string();
        }
        self.parse_index(task_number, Self::remove)
    }

    pub fn print_list(&self) {
        if self.is_empty() {
            printcoln!("[b]Your todo list is empty.");
            return;
        }
        printcoln!("[yellow|b]TODO: ");
        self.task_list.iter().enumerate()
            .for_each(|(i, t)| { printcoln!("[b]{}.[:] {}", i + 1, t.to_string()); });
        printcoln!("[magenta|b]DONE: ");
        self.done_list.iter().enumerate()
            .for_each(|(i, t)| { printcoln!("[b]{}.[:] [s]{}", i + 1, t.to_string()); });
    }

    fn mark_as_done(&mut self, index: i32) -> bool {
        if self.task_list.is_empty() || index as usize > self.task_list.len() || index < 0 {
            printcoln!("[red]Please input valid task index.[:] \
            You can check the list using command `[b]list[:]`.");
            return false;
        }
        let u_index = index as usize;
        self.task_list[u_index].set_done(true);
        let task = self.task_list[u_index].clone();
        self.done_list.push(task);
        self.task_list.remove(u_index);
        true
    }
    pub fn mark_as_done_cui(&mut self, cmd: &Vec<&str>) -> Option<i32> {
        if cmd.len() < 2 {
            printcoln!("[red|b] Please input task to mark as done.");
            return None
        }
        self.parse_index(cmd[1].to_string(), Self::mark_as_done)
    }

    fn mark_undone(&mut self, index: i32) -> bool {
        if self.done_list.is_empty() || index as usize > self.done_list.len() || index < 0 {
            printcoln!("[red]Please input valid task index.[:] \
            You can check the list using command `[b]list[:]`.");
            return false;
        }
        let u_index = index as usize;
        self.done_list[u_index].set_done(false);
        let task = self.done_list[u_index].clone();
        self.task_list.push(task);
        self.done_list.remove(u_index);
        true
    }
    pub fn mark_undone_cui(&mut self, cmd: &Vec<&str>) -> Option<i32> {
        if cmd.len() < 2 {
            printcoln!("[red|b] Please input task to mark as done.");
            return None
        }
        self.parse_index(cmd[1].to_string(), Self::mark_undone)
    }

    pub fn clear_done(&mut self) {
        self.done_list.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.task_list.is_empty() && self.done_list.is_empty()
    }

    fn parse_index(&mut self, string: String, function: fn(&mut Self,i32) -> bool) -> Option<i32> {
        match string.trim().parse::<i32>() {
            Ok(i) => {
                if function(self, i - 1) {
                    return Some(i)
                }
                None
            },
            Err(_) => {
                printcoln!("[red|b]invalid input[:]. function use task number as input.");
                None
            }
        }
    }
}