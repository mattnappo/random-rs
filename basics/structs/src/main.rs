#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };

    println!("area of rect is {}", r.area());
    println!("rect is {:#?}", r);
}
