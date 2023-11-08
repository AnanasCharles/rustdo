use colored::*;
use std::time::Duration;

pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub struct TaskList {
    pub tasks: Vec<Task>
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) {
        let index = if index <= 0 { 1000 } else { index - 1 };
        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red().bold());
            // Sleep for 2 seconds
            std::thread::sleep(Duration::from_secs(2));
        } else {
            self.tasks.remove(index);
        }
    }

    pub fn complete_task(&mut self, index: usize) {
        let index = if index <= 0 { 1000 } else { index - 1 };
        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red().bold());
            std::thread::sleep(Duration::from_secs(2));
        } else {
            self.tasks[index].completed = true;
        }
    }

    pub fn uncheck_task(&mut self, index: usize) {
        let index = if index <= 0 { 1000 } else { index - 1 };
        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red().bold());
            std::thread::sleep(Duration::from_secs(2));
        } else {
            self.tasks[index].completed = false;
        }
    }

    pub fn view_tasks(&self) {
        let title = "TASKS".bold();
        println!("{}", title);
        for (i, task) in self.tasks.iter().enumerate() {
            let i = i + 1;
            let mark = if task.completed { "✓".green() } else { "✗".red() };
            println!("{}: {} - {}", i, task.description, mark);
        }
        println!("\n\n");
    }
}