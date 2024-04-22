fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);

    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("third: {}", third);

    let third = v.get(2);
    match third {
        Some(third) => println!("third: {}", third),
        None => println!("nothing"),
    }

    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }
}
