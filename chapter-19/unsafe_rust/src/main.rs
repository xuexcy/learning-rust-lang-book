use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        return (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        );
    }
}

// 调用 c 函数 abs
extern "C" {
    fn asb(input: i32) -> i32;
}

// 给 c 调用
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!("left: {:?}", left);
    println!("right: {:?}", right);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
