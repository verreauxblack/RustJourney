use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufRead, Write};

static FILE_PATH: &str = "todo.txt";

#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

pub fn add_task(description: String) {
    let task = Task::new(description);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Failed to open file.");

    writeln!(file, "{},{}", task.description, task.completed)
        .expect("Failed to write to file.");

    println!("Task added successfully.");
}

pub fn list_task() {
    let file = File::open(FILE_PATH).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line.");
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let description = parts[0];
            let completed = parts[1].parse::<bool>().unwrap_or(false);
            let status = if completed { "✓" } else { " " };
            println!("{}: [{}] {}", index + 1, status, description);
        }
    }
}

pub fn complete_task(task_num: usize) {
    let file = File::open(FILE_PATH).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut tasks: Vec<Task> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let description = parts[0].to_string();
            let completed = parts[1].parse::<bool>().unwrap_or(false);
            tasks.push(Task {
                description,
                completed
            })
        }
    }

    if task_num > 0 && task_num <= tasks.len() {
        tasks[task_num - 1].completed = true;
        let mut file = File::create(FILE_PATH).expect("Failed to create file.");
        for task in tasks {
            writeln!(file, "{}, {}", task.description, task.completed)
                .expect("Failed to write to file");
        }
        println!("Task {} marked as complete.", task_num);
    } else {
        println!("Invalid task number.");
    }
}