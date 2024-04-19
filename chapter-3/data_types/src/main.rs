use std::io;

fn arr() {
    let a = [1, 2, 3, 4, 5];
    println!("please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("index entered was not a number");
    let element = a[index];
    println!("The value of the element as index {index} is: {element}");

}
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;  // 解构 destructuring
    let t = tup.1;
    println!("{y}");
    println!("{t}");
    let a: [i32; 5] = [1,2,3,4,5];  // tuple
    let b = [3; 5];  // [3, 3, 3, 3, 5]

    arr();
}
