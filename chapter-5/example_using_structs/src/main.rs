#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_v2(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
    println!("The area of the rectangle is {} square pixels.", area_v3(&rect2));

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_v3(rect: &Rectangle) -> u32 {
    return rect.height * rect.width;
}
