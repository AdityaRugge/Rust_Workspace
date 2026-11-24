fn write_array(x1: &mut [u8; 5]) {
    x1[0] = 10;
    println!("Processing array: {:?}", x1);
}

fn main() {
    let mut x: [u8; 5] = [1, 2, 3, 4, 5];
    write_array(&mut x);
    print!("from main: {:?}", x);
}

