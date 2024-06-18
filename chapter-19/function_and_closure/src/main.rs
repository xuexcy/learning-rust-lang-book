
fn add_one(x: i32) -> i32 {
    return x + 1;
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    return f(arg) + f(arg);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    println!("Hello, world!");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(
        |i| i.to_string()).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
