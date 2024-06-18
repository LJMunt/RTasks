mod tasks;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::exit;
use std::{env, io};
use tasks::TaskList;

enum Command {
    Add,
    View { task_id: usize },
    ListUncompleted,
    Complete { task_id: usize },
    ListCompleted,
    ListPrioritized,
    Remove {task_id: usize},
    Help,
    Exit,
}

impl Command {
    fn from_input(input: String) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["add"] => Some(Command::Add),
            ["view", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::View { task_id }),
            ["lc"] => Some(Command::ListCompleted),
            ["lu"] => Some(Command::ListUncompleted),
            ["lp"] => Some(Command::ListPrioritized),
            ["cpl", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::Complete { task_id }),
            ["remove", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::Remove {task_id}),
            ["help"] => Some(Command::Help),
            ["exit"] => Some(Command::Exit),
            _ => None,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Command::Add => "add - Adds a new task. Requires a title and a description",
            Command::View { .. } => "view <id> - Views the task with the entered id.",
            Command::ListCompleted => "lc - Lists all completed tasks",
            Command::ListUncompleted => "lu - Lists all uncompleted tasks",
            Command::ListPrioritized => "lp - lists all tasks in order of priority",
            Command::Complete { .. } => "cpl <id> - Completes the task with the entered id",
            Command::Remove { .. } => "remove <id> - Deletes the Task with the entered id",
            Command::Help => "help - Shows this help dialog",
            Command::Exit => "exit - Saves the task list and exits.",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage {} /path/to/csv", args[0]);
        exit(1);
    }
    let csv_path = &args[1];
    println!("Welcome to RTasks! Type help for a list of commands.");
    let mut task_list = TaskList::load_from_csv(&csv_path).unwrap_or_else(|err| {
        eprintln!("Error loading from CSV: {}", err);
        TaskList::new("Initial".to_string())
    });

    loop {
        print!("Command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(command) = Command::from_input(input) {
            match command {
                Command::Add => task_list.add_task(),
                Command::View { task_id } => task_list.view_task(task_id),
                Command::ListCompleted => task_list.list_completed_tasks(),
                Command::ListUncompleted => task_list.list_uncompleted_tasks(),
                Command::ListPrioritized => task_list.list_priorities(),
                Command::Complete { task_id } => task_list.complete_task(task_id),
                Command::Remove { task_id} => task_list.remove_task(task_id),
                Command::Exit => end_rtasks(&task_list, csv_path),
                Command::Help => help_menu(),
            }
        }
    }
}

fn end_rtasks(list: &TaskList, path: &String) {
    if let Err(err) = list.save_to_csv(path) {
        eprintln!("Error saving: {}", err);
        exit(0);
    }
    println!("Thank you. {} has been saved.", list.title);
    exit(0);
}

fn help_menu() {
    println!("Available Commands:");
    println!("{}", Command::Add.as_str());
    println!("{}", Command::View { task_id: 0 }.as_str());
    println!("{}", Command::ListCompleted.as_str());
    println!("{}", Command::ListUncompleted.as_str());
    println!("{}", Command::ListPrioritized.as_str());
    println!("{}", Command::Complete { task_id: 0 }.as_str());
    println!("{}", Command::Remove {task_id: 0}.as_str());
    println!("{}", Command::Exit.as_str());
    println!("{}", Command::Help.as_str());
}
