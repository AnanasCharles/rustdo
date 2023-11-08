use std::io;
use colored::*;

mod tasklist;
mod terminal;
mod functions;

fn main() {
    terminal::clear_term();
    println!("{} - {}", "RustDo for your To-Do's!".bold().underline(), "Data are not persistent (yet?)");
    println!("\n");
    
    let mut task_list = tasklist::TaskList { tasks: Vec::new() };

    loop {
        task_list.view_tasks();
        println!("What would you like to do?");
        println!("{}", "1. Add task".yellow());
        println!("{}", "2. Remove task".red());
        println!("{}", "3. Complete task".bright_green());
        println!("{}", "4. Uncheck task".blink());
        println!("{}", "5. Quit".bold().red());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        terminal::clear_term();


        match input.trim() {
            "1" => {
                functions::add_task(&mut task_list)
            }
            "2" => {
                functions::remove_task(&mut task_list);
            }
            "3" => {
                functions::complete_task(&mut task_list);
            }
            "4" => {
                functions::uncheck_task(&mut task_list);
            }
            "5" => {
                break;
            }
            _ => {
                println!("{}", "Invalid input".bold().red());
            }
        }
    }

}