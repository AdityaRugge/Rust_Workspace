fn process_string(x: String) {
    println!("Processing string: {}", x);
}

fn main() {
    let x : String = From::from("Hello");
    process_string(x);
    //print!("from main: {}", x);
}

