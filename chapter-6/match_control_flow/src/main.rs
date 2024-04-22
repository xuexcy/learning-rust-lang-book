enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 1,
        Coin::Dime => 1,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn dice() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 通配模式
        other => move_player(other),
        // 或占位符
        // _ => reroll(),
        // do nothing
        // _ => ()
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}

fn if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
    }
}
