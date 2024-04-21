fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    reference_twice();
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn reference_twice_no() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    // println!("{}, {}", r1, r2);  // no
}

fn reference_twice() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}

fn reference_mutable_and_not() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, {}", r1, r2, r3);
}

fn dangle() -> &String {
    let s = String::from("hello");
    return &s;  // NO
}

fn no_dangle() -> String {
    let s = String::from("hello");
    return s;  // YES
}

