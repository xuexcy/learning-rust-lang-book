use default_macro::Default;
use default_macro_derive::MyDefault;

#[derive(MyDefault, Debug)]
struct SomeData (u32,String);

#[derive(MyDefault, Debug)]
struct User {
    name: String,
    data: SomeData,
}

fn main() {
    println!("Hello, world!");
    println!("SomeData default: {:?}", SomeData::default());
    println!("User default: {:?}", User::default());
}
