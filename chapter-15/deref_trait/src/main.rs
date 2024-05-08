use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(5);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // 强制转换，相当于 let name: &str = m.deref().deref();
    let name: &str = m.deref().deref();
    hello(name);
    let name: &str = &(*m)[..];  // 如果rust没用强制转换, 需要[..]获取整个字符串的slice
    hello(name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn hello(name: &str) {
    println!("hello, {name}");
}
