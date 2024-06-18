use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct AA;

#[derive(HelloMacro)]
struct BB;


fn main() {
    AA::hello_macro();
    BB::hello_macro();
}
