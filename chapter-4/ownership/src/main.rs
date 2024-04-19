fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{s}");
    } // drop(s)
    fn_move();
    fn_clone();
    ownership_and_functions();
}

fn fn_move() {
    let s1 = String::from("hello");  // 数据在堆上, s1存着ptr、len、capacity
    let s2 = s1;
    // 按前面drop的说法，在s1、s2出了作用域会drop(s1)、drop(s2), 出现double free
    // 实际上s1已不再有效,不需要drop(s1),解决了double free的问题
    // error
    // println!("{}, world", s1);
    println!("{}, world", s2);
}

fn fn_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_and_functions() {
    let s = String::from("hello");
    takes_ownership(s);
    // 这里无法再使用s
    let x = 5;
    makes_copy(x);
    // 这里还可以使用x
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // drop(some_string)

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string  = String::from("yours");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}
