use std::io;
use std::io::Write;
use std::process::exit;

struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool
}

struct TaskList {
    id_tracker: usize,
    title: String,
    list: Vec<Task>
}

impl Task {
    pub fn new(id: usize, title: String, description: String) -> Self {
        Self {id, title, description, completed: false}
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl TaskList {

    pub fn new(title: String) -> Self {
        Self {id_tracker: 1, title, list: vec![] }
    }

    pub fn add_task(&mut self) {
        println!("--------------");
        let new_title = Self::create_title();
        println!("Title: {}", &new_title);
        let new_description = Self::create_description();
        println!("Description: {}",&new_description);
        self.list.push(Task::new(self.id_tracker, new_title, new_description));
        println!("Created Task {} in List {}",&self.id_tracker, &self.title);
        self.id_tracker += 1;
    }

    fn create_title() -> String {
        loop {
            print!("Enter the Title: ");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            if title.len() > 0 && title.len() < 24 {
                return title
            } else {
                println!("Title can't be empty or longer than 24 characters.");
            }
        }
    }

    fn create_description() -> String {
        loop {
            print!("Enter the Description: ");
            io::stdout().flush().unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            if description.len() > 0 {
                return description
            } else {
                println!("Description can't be empty.")
            }
        }
    }

    pub fn list_completed_tasks(&self) {
        for task in self.list.iter().filter(|t| t.completed) {
            println!("{}: {}",task.id,task.title);
        }
    }

    pub fn list_uncompleted_tasks(&self) {
        for task in self.list.iter().filter(|t| !t.completed) {
            println!("{}: {}",task.id,task.title);
        }
    }

    pub fn complete_task(&mut self, id:usize) {
        if let Some(task) = self.find_task_by_id(id) {
            if task.completed == false {
                task.complete();
            } else {
                println!("Task {} already complete",id);
            }
        } else {
            println!("Task {} does not exist",id);
        }
    }

    pub fn view_task(&mut self, id:usize) {
        if let Some(task) = self.find_task_by_id(id) {
            println!("Task {}: {}",task.id, task.title);
            println!("{}",task.description);
            if task.completed {
                println!("Completed.");
            } else {
                println!("Incomplete");
            }
        } else {
            println!("Task {} not found",id)
        }

    }

    fn find_task_by_id(&mut self, id:usize) -> Option<&mut Task> {
        self.list.iter_mut().find(|task| task.id == id)
   }
}

enum Command {
    Add,
    View {task_id: usize},
    ListUncompleted,
    Complete {task_id: usize},
    ListCompleted,
    Help,
    Exit,
}

impl Command {
    fn from_input(input: String) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["add"] => Some(Command::Add),
            ["view", task_id] => task_id.parse().ok().map(|task_id|Command::View {task_id}),
            ["lc"] => Some(Command::ListCompleted),
            ["lu"] => Some(Command::ListUncompleted),
            ["cpl",task_id] => task_id.parse().ok().map(|task_id|Command::Complete {task_id}),
            ["help"] => Some(Command::Help),
            ["exit"] => Some(Command::Exit),
            _ => None,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Command::Add => "add - Adds a new task. Requires a title and a description",
            Command::View {..} => "view <id> - Views the task with the entered id.",
            Command::ListCompleted => "lc - Lists all completed tasks",
            Command::ListUncompleted => "lu - Lists all uncompleted tasks",
            Command::Complete {..} => "cpl <id> - Completes the task with the entered id",
            Command::Help => "help - Shows this help dialog",
            Command::Exit => "exit - Saves the task list and exits."
        }
    }
}

fn main() {
    println!("Welcome to RTasks! Type help for a list of commands.");
    let mut task_list = TaskList::new("initial".to_string());

    loop {
        print!("Command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(command) = Command::from_input(input) {
            match command {
                Command::Add => task_list.add_task(),
                Command::View {task_id} => task_list.view_task(task_id),
                Command::ListCompleted => task_list.list_completed_tasks(),
                Command::ListUncompleted => task_list.list_uncompleted_tasks(),
                Command::Complete {task_id} => task_list.complete_task(task_id),
                Command::Exit => end_rtasks(&task_list.title),
                Command::Help => help_menu()
            }
        }
    }
}

fn end_rtasks(list_name: &String) {
    println!("Thank you. {} has been saved.",list_name);
    exit(0);
}

fn help_menu() {
    println!("Available Commands:");
    println!("{}", Command::Add.as_str());
    println!("{}",Command::View {task_id: 0}.as_str());
    println!("{}", Command::ListCompleted.as_str());
    println!("{}", Command::ListUncompleted.as_str());
    println!("{}", Command::Complete {task_id: 0}.as_str());
    println!("{}", Command::Exit.as_str());
    println!("{}", Command::Help.as_str());
}