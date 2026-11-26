#[derive(Debug)]

struct Task {
    id: u32,
    description: String,
}

fn change_task( task: &mut Task) {
    task.id = 10;
}

fn main(){

    let mut task = Task {
        id: 1,
        description: String::from("Complete bank transaction \n"),
    };

    change_task(&mut task);
    println!(" After bank transaction \n {:#?}", task); 
}


