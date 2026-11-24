use std::io;

fn main() {
    let mut input: String = String::new();
    println!("Please enter a value: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("User input :{}", input);

    // let x = input;

    // match x.as_str() {
    //         "Hello-rust" => print!("the value is in range: {}", x),
    //         _ => print!("the value is something else"),
    //     };
        
    } 



// fn write_vector(v: &mut Vec<i32>) {
//     v.push(4);
//     v.push(5);
//     print!("from write_vector: {:?}", v);
// }
