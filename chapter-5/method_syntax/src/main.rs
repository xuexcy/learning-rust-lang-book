#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // self: &Self 缩写成 &self
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn width(&self) -> bool {
        return self.width > 0;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_sqr = f64::powi(other.x - self.x, 2);
        let y_sqr = f64::powi(other.y - self.y, 2);
        return f64::sqrt(x_sqr + y_sqr);
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width: it is {}", rect1.width);
    }

    let p1 = Point{ x: 0.0, y: 0.0};
    let p2 = Point{ x: 5.0, y: 6.5};
    println!("{}", p1.distance(&p2));
    println!("{}", (&p1).distance(&p2));


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rec2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rec3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}
