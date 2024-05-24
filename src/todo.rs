use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use dirs::home_dir;
use chrono::{Local, DateTime};
use prettytable::{Table, Row, Cell, row};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub content: String,
    pub created_at: String,
}

impl std::fmt::Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.content, self.created_at)
    }
}

#[derive(Debug)]
pub struct Todo {
    pub items: Vec<TodoItem>,
    file_path: PathBuf,
}

impl Todo {
    pub fn new() -> Todo {
        let mut file_path = home_dir().expect("Could not find home directory");
        file_path.push("todo.json");
        Todo { items: Vec::new(), file_path }
    }

    pub fn add(&mut self, item_content: String) {
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.items.push(TodoItem { content: item_content, created_at });
    }

    pub fn edit(&mut self, idx: usize, new_content: String) -> Result<(), String> {
        if idx < self.items.len() {
            self.items[idx].content = new_content;
            Ok(())
        } else {
            Err("Can Not edit Content for given index".to_string())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index < self.items.len() {
            self.items.remove(index);
            Ok(())
        } else {
            Err("Invalid index".to_string())
        }
    }

    pub fn load() -> Result<Todo, io::Error> {
        let mut todo = Todo::new();
        let file = OpenOptions::new().read(true).open(&todo.file_path);

        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                let items: Vec<TodoItem> = serde_json::from_reader(reader)?;
                todo.items = items;
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                println!("Todo file not found, creating a new one.");
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(todo)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        serde_json::to_writer(file, &self.items)?;
        Ok(())
    }

    pub fn list(&self) {
        let mut table = Table::new();
        table.add_row(row!["Index", "Content", "Created At"]);

        for (index, item) in self.items.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&index.to_string()),
                Cell::new(&item.content),
                Cell::new(&item.created_at.to_string()),
            ]));
        }

        table.printstd();
    }
}
