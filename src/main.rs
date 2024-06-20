mod tasks;
mod commands;
use std::io::Write;
use std::process::{exit};
use std::{env, io};
use tasks::TaskList;
use commands::Command;


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
                Command::ListAll => task_list.list_all_tasks(),
                Command::ListCompleted => task_list.list_completed_tasks(),
                Command::ListUncompleted => task_list.list_uncompleted_tasks(),
                Command::ListPrioritized => task_list.list_priorities(),
                Command::PriorityList { priority } => task_list.list_by_priority(priority.to_string()),
                Command::Complete { task_id } => task_list.complete_task(task_id),
                Command::Edit { task_id } => task_list.edit_task(task_id),
                Command::Remove { task_id } => task_list.remove_task(task_id),
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
    println!("{}", Command::ListAll.as_str());
    println!("{}", Command::ListCompleted.as_str());
    println!("{}", Command::ListUncompleted.as_str());
    println!("{}", Command::ListPrioritized.as_str());
    println!("{}", Command::PriorityList { priority: 0.to_string() }.as_str());
    println!("{}", Command::Complete { task_id: 0 }.as_str());
    println!("{}", Command::Remove { task_id: 0 }.as_str());
    println!("{}", Command::Edit { task_id: 0 }.as_str());
    println!("{}", Command::Exit.as_str());
    println!("{}", Command::Help.as_str());
}
