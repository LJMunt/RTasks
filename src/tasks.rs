use csv::{ReaderBuilder, WriterBuilder};
use serde_derive::{Deserialize, Serialize};
use std::{fmt, io};
use std::fmt::Formatter;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: String,
    priority: Priority,
    completed: bool,
}

pub struct TaskList {
    id_tracker: usize,
    pub(crate) title: String,
    list: Vec<Task>,
    MAX_SIZE: usize,
    ERROR_POS: usize
}
impl Task {
    pub fn new(id: usize, title: String, description: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            description,
            priority,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) {
        println!("{}: {} [{}]", self.id, self.title, self.priority);
    }

    pub fn change_priority(&mut self, new_priority: Priority) {
        self.priority = new_priority
    }
}

impl TaskList {
    pub fn new(title: String) -> Self {
        Self {
            id_tracker: 1,
            title,
            list: vec![],
            MAX_SIZE: 400000,
            ERROR_POS: 400004,
        }
    }
    fn get_max_size(&self) -> &usize {
        return &self.MAX_SIZE;
    }

    fn get_error_pos(&self) -> &usize {
        return &self.ERROR_POS;
    }

    pub fn add_task(&mut self) {
        if &self.list.len() >= self.get_max_size() {
            println!("Maximum number of Tasks reached. Remove some to continue.")
            return;
        }
        println!("--------------");
        let new_title = Self::create_title();
        println!("Title: {}", &new_title);
        let new_description = Self::create_description();
        println!("Description: {}", &new_description);
        let new_priority: Priority = Self::set_priority();
        println!("Priority: {}", &new_priority);
        self.list
            .push(Task::new(self.id_tracker, new_title, new_description, new_priority));
        println!("Created Task {} in List {}", &self.id_tracker, &self.title);
        self.id_tracker += 1;
    }

    fn create_title() -> String {
        loop {
            print!("Enter the Title: ");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();
            if title.len() > 0 && title.len() < 24 {
                return title;
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
                return description;
            } else {
                println!("Description can't be empty.")
            }
        }
    }

    fn set_priority() -> Priority {
        loop {
            print!("Enter the Priority: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<Priority>() {
                Ok(priority) => return priority,
                Err(e) => println!("{}",e),
            }
        }
    }

    pub fn list_completed_tasks(&self) {
        for mut task in self.list.iter().filter(|t| t.completed) {
            task.display();
        }
    }

    pub fn list_uncompleted_tasks(&mut self) {
        for mut task in self.list.iter().filter(|t| !t.completed) {
            task.display();
        }
    }

    pub fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.find_task_by_id(id) {
            if task.completed == false {
                task.complete();
            } else {
                println!("Task {} already complete", id);
            }
        } else {
            println!("Task {} does not exist", id);
        }
    }

    pub fn view_task(&mut self, id: usize) {
        if let Some(task) = self.find_task_by_id(id) {
            println!("Task {}: {}  --  {:?}", task.id, task.title, task.priority);
            println!("{}", task.description);
            if task.completed {
                println!("Completed.");
            } else {
                println!("Incomplete");
            }
        } else {
            println!("Task {} not found", id)
        }
    }

    fn find_task_by_id(&mut self, id: usize) -> Option<&mut Task> {
        self.list.iter_mut().find(|task| task.id == id)
    }

    pub fn load_from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = ReaderBuilder::new().from_path(path)?;
        let mut tasks = Vec::new();
        for result in reader.deserialize() {
            let task: Task = result?;
            tasks.push(task);
        }
        let mut id_tracker = tasks.iter().map(|task| task.id).max().unwrap_or(0);
        id_tracker+=1;
        Ok(TaskList {
            id_tracker,
            title: "Initial".to_string(),
            list: tasks,
        })
    }

    pub fn save_to_csv<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = WriterBuilder::new().from_path(path)?;
        for task in &self.list {
            writer.serialize(task)?;
        }
        let _ = writer.flush();
        Ok(())
    }

    pub fn list_priorities(&mut self) {
        self.list.sort_by(|a,b| b.priority.cmp(&a.priority));
        for mut task in &self.list {
            task.display();
        }
    }

    fn get_task_position(&mut self, id: usize) -> usize {
        if let Some(pos) = self.list.iter().position(|task| task.id == id) {
            pos
        } else {
            400004
        }
    }

    fn remove_task(&mut self, pos: usize) {
        if pos == 400004 {
            println!("Task not found.");
        } else {
            self.list.remove(pos);
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("'{}' is not a valid priority",s)),
        }
    }
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       let priority_str = match self {
           Priority::Low => "Low",
           Priority::Medium => "Medium",
           Priority::High => "High",
           Priority::Critical => "Critical",
       };
        write!(f, "{}",priority_str)
    }
}
