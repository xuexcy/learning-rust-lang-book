fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Hello, world!");

    // v1.iter().map(|x| x + 1); // map后没用使用v1,迭代器是惰性的，所以map并没有实际执行
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
