#[derive(Debug)]
enum USState {
    NewYork,
    NewJersey,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn get_cents(c: &Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(USState::NewJersey) => {
            println!("ITS JERSEY!");
            25
        }
        Coin::Quarter(s) => {
            println!("quarter state is {:?}!", s);
            25
        }
    }
}
fn main() {
    let change: [Coin; 3] = [Coin::Penny, Coin::Penny, Coin::Quarter(USState::NewJersey)];

    println!("value of {:?} is {}", &change[2], get_cents(&change[2]));
}
