mod task;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo-cli <command> [arguments]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task description.");
            } else {
                let description = args[2..].join(" ");
                task::add_task(description);
            }
        }
        "list" => {
            task::list_task();
        }
        "complete" => {
            if args.len() < 3 {
                println!("Please provide the task number to complete.");
            } else {
                let task_num: usize = args[2].parse().expect("Please provide a valid task number.");
                task::complete_task(task_num);
            }
        }
        _ => {
            println!("Unknown command. Use 'add', 'list', or 'complete'.");
        }
    }
}
