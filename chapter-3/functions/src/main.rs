fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(2);
    print_labeled_measurement(1,'你');
    statement_and_expression();
    println!("five: {}", five());
    println!("six: {}", six());
    let x = plus_one(5);
    println!("x: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statement_and_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {y}");
}

fn five() -> i32 {
    5
}
fn six() -> i32 {
    return 6;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
