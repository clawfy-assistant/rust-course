//! # Todo CLI App
//! 
//! โปรเจคจบ: แอพจัดการ Todo List ผ่าน Command Line

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, description: &str) -> Self {
        Task {
            id,
            description: description.to_string(),
            completed: false,
        }
    }
    
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
    
    pub fn add(&mut self, description: &str) -> u32 {
        let id = self.next_id;
        self.tasks.insert(id, Task::new(id, description));
        self.next_id += 1;
        id
    }
    
    pub fn complete(&mut self, id: u32) -> Option<&Task> {
        self.tasks.get_mut(&id).map(|task| {
            task.complete();
            task as &Task
        })
    }
    
    pub fn remove(&mut self, id: u32) -> Option<Task> {
        self.tasks.remove(&id)
    }
    
    pub fn list(&self) -> Vec<&Task> {
        let mut tasks: Vec<_> = self.tasks.values().collect();
        tasks.sort_by_key(|t| t.id);
        tasks
    }
    
    pub fn list_completed(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| t.completed)
            .collect()
    }
    
    pub fn list_pending(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| !t.completed)
            .collect()
    }
}

fn main() {
    let mut todo = TodoList::new();
    
    // Add some tasks
    let id1 = todo.add("Learn Rust basics");
    let id2 = todo.add("Practice ownership");
    let id3 = todo.add("Build a project");
    
    println!("=== All Tasks ===");
    for task in todo.list() {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {}: {}", status, task.id, task.description);
    }
    
    // Complete a task
    todo.complete(id1);
    
    println!("\n=== After completing task {} ===", id1);
    for task in todo.list() {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {}: {}", status, task.id, task.description);
    }
    
    println!("\n=== Pending Tasks ===");
    for task in todo.list_pending() {
        println!("[ ] {}: {}", task.id, task.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut todo = TodoList::new();
        let id = todo.add("Test task");
        assert_eq!(id, 1);
        
        let tasks = todo.list();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Test task");
    }

    #[test]
    fn test_complete_task() {
        let mut todo = TodoList::new();
        let id = todo.add("Test task");
        
        assert!(todo.complete(id).is_some());
        assert!(todo.list()[0].completed);
        
        assert!(todo.complete(999).is_none());
    }

    #[test]
    fn test_remove_task() {
        let mut todo = TodoList::new();
        let id = todo.add("To be removed");
        
        assert!(todo.remove(id).is_some());
        assert!(todo.list().is_empty());
    }

    #[test]
    fn test_list_completed_and_pending() {
        let mut todo = TodoList::new();
        let id1 = todo.add("Task 1");
        let id2 = todo.add("Task 2");
        todo.complete(id1);
        
        assert_eq!(todo.list_completed().len(), 1);
        assert_eq!(todo.list_pending().len(), 1);
    }
}
