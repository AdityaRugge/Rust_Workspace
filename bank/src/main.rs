#[derive(Debug)]

struct Task {
    id: u32,
    description: String,
}

fn print_description(task: &Task) {
    println!("Task ID: {}, Description: {}", task.id, task.description);
}

fn main(){
    let task = Task {
        id: 1,
        description: String::from("Complete the bank transaction"),
    };
    print_description(&task);
    println!("{:#?}", task); 
}


