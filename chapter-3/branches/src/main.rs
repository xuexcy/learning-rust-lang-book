fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let a = loop {
        counter += 1;
        println!("again!");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("break counter*2: {a}");
    loop_labels();
    while_loop();
    for_loop();
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {

        println!("count={count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40];
    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {  // [1, 4)
        println!("{number}");  // not print 4
    }
}
