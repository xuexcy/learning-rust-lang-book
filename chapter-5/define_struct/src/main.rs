struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体: 有类型名字段类型，没用字段名
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;  // 类单元结构体(unit-like structs)



fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("xuechengyun"),
        email: String::from("xuechengyunxue@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("143309494@qq.com");
    println!("Hello, world!");

    let black = Color(0, 0, 1);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    };
}

fn build_user_v2(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}

fn build_user_v3() -> User {
    let user1 = User {
        email: String::from("a"),
        username: String::from("b"),
        active: true,
        sign_in_count: 1,
    };
    return User {
        email: String::from("c"),
        ..user1
    };
    // user1的username被移动到返回值的username中，不能再使用
}
