fn main() {
    println!("Hello, world!");
    assert_eq!(-1, !0); // -1 = -0 -1;
    assert_eq!(-6, !5); // -6 = -5 -1;
    assert_eq!(-11, !10); // -11 = -10 -1;
    assert!(1 != 2); // true
    assert_eq!(1, 10 % 3); // 1
    let mut a = 10;
    a %= 3; // 1
    assert_eq!(1, a);
    let b = &a; // borrow
    println!("{}", b);

    assert_eq!(0, 1 & 2);
    assert_eq!(1, 1 & 3);

    let x = 3;
    let y = &x;
    assert_eq!(3, x);
    // assert_eq!(3, y);
    assert_eq!(3, *y);

    // 0, 1, 2 .. 10
    for i in 0..=10 {
        print!("{i} ");
    }
    println!("");

    // 0, 1, 2 ..9
    for i in 0..10 {
        print!("{i} ");
    }
    println!("");

    for _i in 0..100 {
        do_something();
    }

    // cargo clippy
    let x = 3.141592653;
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);

}

fn do_something() {}
