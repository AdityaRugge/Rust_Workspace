fn main() {
    let mut v:Vec<i32> = vec![1, 2, 3]; //vector of integers
    write_vector(&mut v);
    print!("from main: {:?}", v);
}

fn write_vector(v: &mut Vec<i32>) {
    v.push(4);
    v.push(5);
    print!("from write_vector: {:?}", v);
}
