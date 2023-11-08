use std::process::Command;

pub fn clear_term() {
    if cfg!(windows) {
        Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("Something went wrong")
    } else {
        Command::new("clear")
            .status()
            .expect("Something went wrong")
    };
}