fn main() {
    println!("ipv4 enum: {:?}", IpAddrKind::V4);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home addr: {:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback addr: {:?}", loopback);

    let home_enum = IpAddrUseEnum::V4(String::from("127.0.0.1"));
    println!("home enum: {:?}", home_enum);

    let m = Message::Write(String::from("write into message"));
    m.call();

    let some_num = Some(5);
    let absent_num: Option<i32> = None;

    let text: Option<String> = Some("Hello, world!".to_string());
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}, {text_length:?}");

    let some_coin = Coin::Quarter(UsState::Alabama);
    println!("coin penny in cents: {}", value_in_cents(some_coin));

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);

    let x = 0u32;
    match x {
        1 => println!("111"),
        2 => println!("222"),
        _ => println!("let it go") // 下划线可以表示通配剩余穷举值
    }

    let config_max = Some(3u32);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // 上下两句等价
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(4) = config_max {
        println!("x is 3")
    } else  {
        println!("x is not 3")
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrUseEnum {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("go into Message.call")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 这行如果没有写，则不能覆盖到所有Option的可能情况，编译失败！
        Some(i) => Some(i + 1),
    }
}

fn decide_roll(x: u8) -> i8 {
    match x {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 这里用下划线通配其他的情况，无需引用match的变量
    }
}

fn add_fancy_hat() -> i8 {
    1
}
fn remove_fancy_hat() -> i8 {
    2
}
fn reroll() -> i8 {
    3
}
