use colored::*;

use crate::tasklist::{Task, TaskList};
use crate::terminal;
use std::io;

pub fn add_task(task_list: &mut TaskList) {
    task_list.view_tasks();
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    task_list.add_task(Task {
        description: description.trim().to_string(),
        completed: false,
    });
    terminal::clear_term();
    println!("{}", "Task added".green().bold());
}

pub fn remove_task(task_list: &mut TaskList) {
    task_list.view_tasks();
    println!("Enter task index:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index = index
        .trim()
        .parse::<usize>()
        .unwrap_or(1000);
        // .expect("Failed to parse index");
    task_list.remove_task(index);
    terminal::clear_term();
    println!("{}", "Task removed".bright_red().bold());
}

pub fn complete_task(task_list: &mut TaskList) {
    task_list.view_tasks();
    println!("Enter task index:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();
    let index = index
        .trim()
        .parse::<usize>()
        // .expect("Failed to parse index");
        .unwrap_or(1000);
    task_list.complete_task(index);
    terminal::clear_term();
}

pub fn uncheck_task(task_list: &mut TaskList) {
    task_list.view_tasks();
    println!("Enter task index:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();
    let index = index
        .trim()
        .parse::<usize>()
        // .expect("Failed to parse index");
        .unwrap_or(1000);
    task_list.uncheck_task(index);
    terminal::clear_term();
}