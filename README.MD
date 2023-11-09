# RustDo - A Rust Terminal To-Do App

RustDo is a simple terminal-based to-do list application written in Rust. It allows you to manage your tasks efficiently without the need for a graphical user interface.


## Features

- Add tasks.
- Mark tasks as completed.
- Remove tasks from your to-do list.
- Tasks are automatically saved to tasks.txt
- Tasks are automatically loaded on start from tasks.txt


## Installation

1. Make sure you have Rust and Cargo installed. If not, you can download and install them from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).

2. Clone this repository using:
```sh
git clone https://github.com/AnanasCharles/rustdo.git
```

3. Change the working directory to the RustDo folder:
```sh
cd rustdo
```

4. Build the app:
```sh
cargo build --release
```

5. Run the app:
```sh
cargo run --release
```


## Usage

- Start the app and use the menu to perform desired action:
    - Add a new task.
    - Remove a task.
    - Mark a task as completed.
    - Uncheck a task.

- Your tasks are automatically saved to a file named 'tasks.txt' in the project directory. This means you can close and relaunch the app at any time, and your tasks will be preserved.


## Contribute

Contributions are welcome! If you have any ideas for enhancements, find bugs or want to contribute in any way, feel free to open issues, submit pull requests, or suggest improvements. I'd appreciate your feedback.


## Acknowledgments

- This project is a simple example of a Rust terminal application.
- Inspired by common to-do list applications.

Happy task management with RustDo!