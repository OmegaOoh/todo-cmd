mod task;

use std::fs;
use std::fs::{remove_file, OpenOptions};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use crate::todo_list::task::Task;
use fancy::{colorize, printcol, printcoln};

pub struct ToDoList {
    task_list: Vec<Task>,
}

impl ToDoList {
    pub(crate) fn new() -> Self {
        Self { task_list: Vec::new()}
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
        let task = Task::new(title.clone(), false);
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
        let mut done_buf = Vec::<String>::new();
        let mut undone_buf = Vec::<String>::new();
        self.task_list.iter().enumerate()
            .for_each(|(i, t)| {
                let text = colorize!("[b]({})[:] {}", i + 1, t.to_string());
                if t.get_done() {
                    done_buf.push(text);
                } else {
                    undone_buf.push(text);
                }
            });
        printcoln!("[yellow|b]TODO: ");
        undone_buf.iter().for_each(|s| { printcoln!("{}",s.to_string());});
        printcoln!("[magenta|b]DONE: ");
        done_buf.iter().for_each(|s| { printcoln!("{}",s.to_string());});
    }

    pub fn mark_as_done_cui(&mut self, cmd: &Vec<&str>) -> Option<i32> {
        if cmd.len() < 2 {
            printcoln!("[red|b] Please input task to mark as done.");
            return None
        }
        self.parse_index(cmd[1].to_string(), Self::mark_as_done)
    }

    pub fn mark_undone_cui(&mut self, cmd: &Vec<&str>) -> Option<i32> {
        if cmd.len() < 2 {
            printcoln!("[red|b] Please input task to mark as done.");
            return None
        }
        self.parse_index(cmd[1].to_string(), Self::mark_undone)
    }

    pub fn clear_done(&mut self) {
        let mut loc_to_rem = Vec::<usize>::new();
        self.task_list.iter().enumerate().for_each(|(i,t)|
        {
            if t.get_done() {
                loc_to_rem.push(i)
            }
        });
        loc_to_rem.reverse();
        loc_to_rem.iter().for_each(|i| { self.task_list.remove(*i); })
    }

    pub fn is_empty(&self) -> bool {
        self.task_list.is_empty() && self.task_list.is_empty()
    }

    pub fn read(&mut self) {
        if !Path::new("task").exists() {
            return;
        }
        let content = fs::read_to_string("task");
        let binding = content.unwrap();
        let task_list = binding.split("\n").collect::<Vec<&str>>();
        task_list.iter().enumerate().for_each(|(_i, t)| {
            let binding = t.to_string();
            let task_detail = binding.split(",").collect::<Vec<&str>>();
            self.task_list.push(Task::new(task_detail[0].to_string(),
                                          task_detail[1].parse::<bool>()
                                              .expect("Invalid File Format")));
        })
    }

    pub fn save(&mut self) {
        if Path::new("task").exists() {
            remove_file("task").unwrap();
        }
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open("task")
            .unwrap();
        let mut contents = Vec::<String>::new();
        self.task_list.iter().enumerate().for_each(
            |(_i,t)|
                {
                    contents.push(t.to_string() + "," + &*t.get_done().to_string());
                }
        );
        let str = contents.join("\n");
        file.write(str.as_bytes()).unwrap();
    }

    fn parse_index(&mut self, string: String, function: fn(&mut Self,i32) -> bool) -> Option<i32> {
        match string.trim().parse::<i32>() {
            Ok(i) => {
                if function(self, i-1) {
                    return Some(i-1)
                }
                None
            },
            Err(_) => {
                printcoln!("[red|b]invalid input[:]. function use task number as input.");
                None
            }
        }
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

    fn mark_state(&mut self, index: i32, is_done: bool) -> bool {
        if self.task_list.is_empty() || index as usize > self.task_list.len() || index < 0 {
            printcoln!("[red]Please input valid task index.[:] \
            You can check the list using command `[b]list[:]`.");
            return false;
        }
        let u_index = index as usize;
        self.task_list[u_index].set_done(is_done);
        true
    }

    fn mark_as_done(&mut self, index: i32) -> bool {
        self.mark_state(index, true)
    }

    fn mark_undone(&mut self, index: i32) -> bool {
        self.mark_state(index, false)
    }
}