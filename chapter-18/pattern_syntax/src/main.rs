use std::os::unix::raw::gid_t;

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x  {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } =  p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On th x axis at {x}"),
        Point { x: 0, y} => println!("On th x axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move x: {x}, y: {y}");
        }
        Message::Write(text) => {
            println!("Text {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color r: {r}, g: {g}, b: {b}");
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum MessageV2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = MessageV2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        MessageV2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Color r: {r}, g: {g}, b: {b}");
        }
        MessageV2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Color h: {h}, s: {s}, v: {v}");
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is : {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
    let _x = 5;

    let s = Some(String::from("hello"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s);

    struct PointV2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = PointV2 { x: 0, y: 0, z: 0 };
    match origin {
        PointV2 { x, .. } => {
            println!("x is {x}");
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum MyMessage {
        Hello { id: i32 },
    }
    let msg = MyMessage::Hello { id: 5 };
    match msg {
        MyMessage::Hello {
            id: id_variable @ 3..=7,
        } => { println!("Found an id in range: {id_variable}"); }
        MyMessage::Hello {
             id: 10..=12
        } => { println!("Found an id in another range"); }
        _ => (),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only use the y: {y}");
}
