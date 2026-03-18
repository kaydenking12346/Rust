use std::io;

enum ToDoAction {
    Add,
    Delete,
    View,
}

fn ask_choice() -> ToDoAction {
    println!("Choose an action: ");
    println!("1 - Add a task");
    println!("2 - Delete a task");
    println!("3 - View tasks");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => ToDoAction::Add,
        "2" => ToDoAction::Delete,
        "3" => ToDoAction::View,
        _ => {
            println!("Invalid choice, try again!");
            ask_choice() // recursively ask again
        }
    }
}

fn add_task(list: &mut Vec<String>) {
    let mut task = String::new();
    println!("Enter task to add:");
    io::stdin().read_line(&mut task).expect("Failed to read input");
    list.push(task.trim().to_string());
    println!("Task added!");
}

fn delete_task(list: &mut Vec<String>) {
    if list.is_empty() {
        println!("No tasks to delete!");
        return;
    }

    println!("Your tasks:");
    for (i, task) in list.iter().enumerate() {
        println!("{}: {}", i + 1, task);
    }

    let mut input = String::new();
    println!("Enter task number to delete:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= list.len() {
            list.remove(index - 1);
            println!("Task deleted!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Please enter a valid number!");
    }
}

fn view_tasks(list: &Vec<String>) {
    if list.is_empty() {
        println!("No tasks yet!");
    } else {
        println!("Your tasks:");
        for (i, task) in list.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    loop {
        let action = ask_choice();

        match action {
            ToDoAction::Add => add_task(&mut todo_list),
            ToDoAction::Delete => delete_task(&mut todo_list),
            ToDoAction::View => view_tasks(&todo_list),
        }

        println!("-------------------------");
    }
}