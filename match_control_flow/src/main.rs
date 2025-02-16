fn main() {
    let alaska_coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(alaska_coin);
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}
#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}
enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("the quarter is from {state:?}");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
