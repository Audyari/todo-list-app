use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    created_at: DateTime<Local>,
}

struct TodoList {
    tasks: HashMap<usize, Task>,
    next_id: usize,
    file_path: PathBuf,
}

impl TodoList {
    fn new(file_path: PathBuf) -> io::Result<Self> {
        let mut todo = Self {
            tasks: HashMap::new(),
            next_id: 1,
            file_path,
        };
        
        todo.load()?;
        Ok(todo)
    }

    fn load(&mut self) -> io::Result<()> {
        if !self.file_path.exists() {
            return Ok(());
        }

        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if !contents.is_empty() {
            let tasks: Vec<Task> = serde_json::from_str(&contents)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            
            for task in tasks {
                self.next_id = self.next_id.max(task.id + 1);
                self.tasks.insert(task.id, task);
            }
        }

        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let tasks: Vec<&Task> = self.tasks.values().collect();
        let json = serde_json::to_string_pretty(&tasks)?;
        
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn add_task(&mut self, description: String) -> io::Result<()> {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            created_at: Local::now(),
        };
        
        self.tasks.insert(task.id, task);
        self.next_id += 1;
        self.save()
    }

    fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|t| t.id);
        tasks
    }

    fn complete_task(&mut self, id: usize) -> io::Result<bool> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = true;
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn delete_task(&mut self, id: usize) -> io::Result<bool> {
        if self.tasks.remove(&id).is_some() {
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { 
        /// The task description
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as complete
    Complete {
        /// The ID of the task to complete
        id: usize,
    },
    /// Delete a task
    Delete {
        /// The ID of the task to delete
        id: usize,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| ".".into())
        .join("todo-list-app");
    let data_file = data_dir.join("tasks.json");
    
    let mut todo = TodoList::new(data_file)?;
    
    match cli.command {
        Commands::Add { description } => {
            todo.add_task(description)?;
            println!("Task added successfully!");
        }
        Commands::List => {
            let tasks = todo.list_tasks();
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!("ID  | Completed | Description");
                println!("----|-----------|------------");
                for task in tasks {
                    let status = if task.completed { "[✓]" } else { "[ ]" };
                    println!("{:3} | {} | {}", task.id, status, task.description);
                }
            }
        }
        Commands::Complete { id } => {
            if todo.complete_task(id)? {
                println!("Task {} marked as complete!", id);
            } else {
                eprintln!("Error: Task with ID {} not found.", id);
            }
        }
        Commands::Delete { id } => {
            if todo.delete_task(id)? {
                println!("Task {} deleted successfully!", id);
            } else {
                eprintln!("Error: Task with ID {} not found.", id);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_todo() -> (TodoList, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_tasks.json");
        (TodoList::new(file_path).unwrap(), temp_dir)
    }

    #[test]
    fn test_add_task() {
        let (mut todo, _temp) = create_test_todo();
        
        // Test adding a task
        todo.add_task("Test task".to_string()).unwrap();
        
        // Verify task was added
        let tasks = todo.list_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Test task");
        assert!(!tasks[0].completed);
    }

    #[test]
    fn test_complete_task() {
        let (mut todo, _temp) = create_test_todo();
        
        // Add and complete a task
        todo.add_task("Test task".to_string()).unwrap();
        let success = todo.complete_task(1).unwrap();
        
        // Verify completion
        assert!(success);
        let tasks = todo.list_tasks();
        assert!(tasks[0].completed);
        
        // Test completing non-existent task
        let success = todo.complete_task(999).unwrap();
        assert!(!success);
    }

    #[test]
    fn test_delete_task() {
        let (mut todo, _temp) = create_test_todo();
        
        // Add and delete a task
        todo.add_task("Test task".to_string()).unwrap();
        let success = todo.delete_task(1).unwrap();
        
        // Verify deletion
        assert!(success);
        assert!(todo.list_tasks().is_empty());
        
        // Test deleting non-existent task
        let success = todo.delete_task(1).unwrap();
        assert!(!success);
    }

    #[test]
    fn test_persistence() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_tasks.json");
        
        // Create and save tasks
        {
            let mut todo = TodoList::new(file_path.clone()).unwrap();
            todo.add_task("Task 1".to_string()).unwrap();
            todo.add_task("Task 2".to_string()).unwrap();
            todo.complete_task(1).unwrap();
        }
        
        // Reload and verify
        let todo = TodoList::new(file_path).unwrap();
        let tasks = todo.list_tasks();
        
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].description, "Task 1");
        assert!(tasks[0].completed);
        assert_eq!(tasks[1].description, "Task 2");
        assert!(!tasks[1].completed);
    }

    #[test]
    fn test_task_ordering() {
        let (mut todo, _temp) = create_test_todo();
        
        // Add tasks in reverse order
        todo.add_task("Task 2".to_string()).unwrap();
        todo.add_task("Task 1".to_string()).unwrap();
        
        // Verify they're listed in ID order
        let tasks = todo.list_tasks();
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }
}
