fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    // can't use s1 here
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    let hello = String::from("hello");
    let s = &hello[0..4];
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }

}
