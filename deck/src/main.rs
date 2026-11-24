

fn main() {
    let x = 51;

    match x {
            1 | 2 | 3 | 4 | 5 => print!("the value is in range: {}", x),
            _ => print!("the value is something else"),
        };
        
    } 



// fn write_vector(v: &mut Vec<i32>) {
//     v.push(4);
//     v.push(5);
//     print!("from write_vector: {:?}", v);
// }
