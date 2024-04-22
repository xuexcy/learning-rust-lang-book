enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

struct QuitMessage;  // 类单元结构体 tuple()
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    println!("Hello, world!");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV2::V6(String::from("::1"));

    let home = IpAddrV3::V4(127, 0, 0, 1);

    let some_number = Some(5);  // Option<i32>
    let some_char = Some('e');
    let absent_number: Option<i32>= None;
}

fn route(ip_kind: IpAddrKind) {}
