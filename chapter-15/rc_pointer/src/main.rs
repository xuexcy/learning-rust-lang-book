enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// b -> 3 -> a
//           a -> 5 -> 10 -> Nil
// c -> 4 -> a
fn main() {
    println!("Hello, world!");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count = {}", Rc::strong_count(&a));
    }
    println!("count = {}", Rc::strong_count(&a));
}

