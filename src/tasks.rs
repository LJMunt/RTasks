use std::io;
use std::io::Write;
use std::path::Path;
use csv::{ReaderBuilder, WriterBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool
}

pub struct TaskList {
    id_tracker: usize,
    pub(crate) title: String,
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

    pub fn load_from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = ReaderBuilder::new().from_path(path)?;
        let mut tasks = Vec::new();
        for result in reader.deserialize() {
            let task: Task = result?;
            tasks.push(task);
        }
        let id_tracker = tasks.iter().map(|task| task.id).max().unwrap_or(0);
        Ok(TaskList {
            id_tracker,
            title: "Initial".to_string(),
            list: tasks,
        })
    }

    pub fn save_to_csv<P: AsRef<Path>>(&self, path: P) -> Result<(),Box<dyn std::error::Error>> {
        let mut writer = WriterBuilder::new().from_path(path)?;
        for task in &self.list {
            writer.serialize(task)?;
        }
        let _ = writer.flush();
        Ok(())
    }

}