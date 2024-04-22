use crate::garden::vegetables::Asparagus

pub mod garden; // 相当于c++ #include

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
