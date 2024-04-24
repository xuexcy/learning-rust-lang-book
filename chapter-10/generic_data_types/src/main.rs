struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

struct PointV2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointV2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointV2<X2, Y2>) -> PointV2<X1, Y2> {
        return PointV2 {
            x: self.x,
            y: other.y,
        };
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = integer;
    println!("p.x = {}", p.x());
    println!("float distance_from_origin = {}", float.distance_from_origin());

    let p1 = PointV2 { x: 5, y: 10.4 };
    let p2 = PointV2 { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
