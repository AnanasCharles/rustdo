use colored::*;
use std::fs::metadata;
use std::{time::Duration, fs::File};
use std::io::{Write, Result, BufReader, BufRead};

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
        self.save_tasks("tasks.txt").expect("Failed to save tasks.");
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
        self.save_tasks("tasks.txt").expect("Failed to save tasks.");
    }

    pub fn complete_task(&mut self, index: usize) {
        let index = if index <= 0 { 1000 } else { index - 1 };
        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red().bold());
            std::thread::sleep(Duration::from_secs(2));
        } else {
            self.tasks[index].completed = true;
        }
        self.save_tasks("tasks.txt").expect("Failed to save tasks.");
    }

    pub fn uncheck_task(&mut self, index: usize) {
        let index = if index <= 0 { 1000 } else { index - 1 };
        if index >= self.tasks.len() {
            println!("{}", "Invalid task index".red().bold());
            std::thread::sleep(Duration::from_secs(2));
        } else {
            self.tasks[index].completed = false;
        }
        self.save_tasks("tasks.txt").expect("Failed to save tasks.");
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

    pub fn save_tasks(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(&mut file, "{}\t{}", task.description, task.completed)?;
        }
        Ok(())
    }

    pub fn load_tasks(&mut self, filename: &str) -> Result<()> {
        if metadata(filename).is_err() {
            File::create(filename)?;
        }

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        self.tasks.clear();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() == 2 {
                let description = parts[0].to_string();
                let completed = parts[1].parse::<bool>().unwrap_or(false);
                self.tasks.push(Task {description, completed});
            }
        }

        Ok(())
        
    }
}