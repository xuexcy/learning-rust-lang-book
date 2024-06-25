use std::thread;
use std::time;

use tokio;
use appendix_A::animal::dog::Dog;
use appendix_A::animal::dog::Corgi;
async fn sum_double(n: u32) -> u32 {
    println!("async start");
    let mut double_sum: u32 = 0;
    for i in 0..=n {
        double_sum += i * 2;
    }
    println!("async end");
    return double_sum;
}

trait Draw {
    fn draw(&self) {
        println!("default draw");
    }
}

struct DefaultGraph;
impl Draw for DefaultGraph {}
struct Circle;
impl Draw for Circle {
    fn draw(&self) {
        println!("circle draw");
    }
}

fn draw_it(draw: &dyn Draw) {
    draw.draw();
}

fn call_it(f: fn()) {
    f();
}

static static_var: &str = "bac";

#[tokio::main]
async fn main() {
    let a: i32 = 5;
    let b = a as u32;

    let n = 10000;
    let f = sum_double(n);
    println!("{a}, {b}");
    println!("double_sum[0..{}]: {}", n, f.await);

    let mut i = 0;
    let mut step = 0;
    let mut size = 1;
    loop {  // 1, 3 4, 6 7 8, 10(not this one) 11 12 13, 15 16 17 18 19
        let max = 1000;
        if i > max {
            println!("loop end, step ={step}");
            break;
        }
        for _ in 1..=size {
            i += 1;
            if i > max {
                break;
            }
            if i % 10 == 0 {
                continue;
            }
            print!("{i} ");

            step += 1;
        }
        println!("");
        i += 1; // skip
        size += 1;
    }
    const cc: i32 = 10;

    draw_it(&DefaultGraph {});
    draw_it(&Circle {});

    if cc != 0 {
        println!("if");
    } else {
        println!("else");
    }

    enum Graph {
        Square,
        Line,
    }

    let tiantian = false;

    call_it(|| { println!("function pointer call")});

    let graph = Graph::Square;
    match graph {
        Graph::Square => { println!("this is square")}
        Graph::Line => { println!("this is line")}
    };
    let corgi = Corgi{};
    corgi.species();

    let mut to_move = 3;
    let handle = thread::spawn(move||{
        println!("in_thread before mut, to_move: {to_move}");
        to_move *= 2;
        println!("in_thread after mut, to_move: {to_move}");
    });
    thread::sleep(time::Duration::from_secs(1));
    handle.join().unwrap();
    to_move *= 3;
    println!("out_of_thread, to_move: {}", to_move);
    println!("{static_var}");

    let u = UseUnion { f1: 3 };

    assert!(r#match("foo", "foobar"));
}

trait MyTrait {
    fn new(i: u32) -> Self;
}

struct MyStruct(i32, u32);

impl MyTrait for MyStruct {
    fn new(i: u32) -> Self {
        return MyStruct(0, i);
    }
}

type Int = i32;
struct UseType {
    a: Int,
}

union UseUnion {
    f1: i32,
    f2: u32,
}

fn r#match(needle: &str, haystack: &str) -> bool {
    return haystack.contains(needle);
}
