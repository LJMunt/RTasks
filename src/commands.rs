pub enum Command {
    Add,
    View { task_id: usize },
    ListUncompleted,
    Complete { task_id: usize },
    ListAll,
    ListCompleted,
    ListPrioritized,
    PriorityList { priority: String },
    Edit { task_id: usize },
    Remove { task_id: usize },
    Help,
    Exit,
}

impl Command {
    pub fn from_input(input: String) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["add"] => Some(Command::Add),
            ["view", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::View { task_id }),
            ["la"] => Some(Command::ListAll),
            ["lc"] => Some(Command::ListCompleted),
            ["lu"] => Some(Command::ListUncompleted),
            ["lp"] => Some(Command::ListPrioritized),
            ["pl", priority] => priority
                .parse()
                .ok()
                .map(|priority| Command::PriorityList { priority }),
            ["cpl", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::Complete { task_id }),
            ["remove", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::Remove { task_id }),
            ["edit", task_id] => task_id
                .parse()
                .ok()
                .map(|task_id| Command::Edit { task_id }),
            ["help"] => Some(Command::Help),
            ["exit"] => Some(Command::Exit),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::Add => "add - Adds a new task. Requires a title and a description",
            Command::View { .. } => "view <id> - Views the task with the entered id.",
            Command::ListAll => "la - Lists all tasks that exist.",
            Command::ListCompleted => "lc - Lists all completed tasks",
            Command::ListUncompleted => "lu - Lists all uncompleted tasks",
            Command::ListPrioritized => "lp - lists all tasks in order of priority",
            Command::PriorityList { .. } => "pl <priority> - lists all tasks of the entered priority.",
            Command::Complete { .. } => "cpl <id> - Completes the task with the entered id",
            Command::Edit { .. } => "edit <id> - Changes the title or description of a task",
            Command::Remove { .. } => "remove <id> - Deletes the Task with the entered id",
            Command::Help => "help - Shows this help dialog",
            Command::Exit => "exit - Saves the task list and exits.",
        }
    }
}