fn main() {
    println!("Hello, world!");
    let s = String::from("hell word");
    let i = first_word(&s);
    println!("{}", i);
    let hell = &s[0..4];
    let word = &s[5..9];
    println!("{} {}", &hell, &word);
    println!("{}", find_first_word(&s));
    let s = String::from("hellworld");
    println!("{}", find_first_word(&s));

    let my_string = String::from("hello world");
    let word = find_first_word_v2(&my_string[0..6]);
    let word = find_first_word_v2(&my_string[..]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn find_first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
