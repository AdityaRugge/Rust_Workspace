fn write_array(mut x: [u8; 5]) {
    x[0] = 10; // This line will cause a compile-time error because x is not mutable
    println!("Processing array: {:?}", x);
}

fn main() {
    let x: [u8; 5] = [1, 2, 3, 4, 5];
    write_array(x);
    print!("from main: {:?}", x);
}

