
mod ip;

fn main() {



    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number: i32 = 5;
    let absent_number: Option<i32> = None;
    let y = some_number +  absent_number.unwrap_or(0);
    println!("{:?}", y);

    
    let c = Coin::Dime;
    println!("{}", value_in_cents(c));
    
    let a_max = Some(10);
    if let Some(max) = a_max {
        println!("The max is {}", max);
    }
    
    match a_max {
        Some(val) => println!("The value is {}", val),
        None => println!("No value"),
    }
}

pub enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
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
       match self {
           Message::Write(text) => println!("{}", text),
           Message::ChangeColor(x, y, z) => println!("{} {} {}", x, y, z),
           Message::Quit => println!("Quit"),
           Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
       }
    }

}

enum Coin {
    Penny,
    Nickel,
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        c => 3
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("Received {}", i);
            return Some(i+1);
        }
    }
}

fn plus_one2(x: Option<i32>) -> Option<i32> {
    let Some(v) = x else {
        return None;
    };
    
    return Some(v+1);
}