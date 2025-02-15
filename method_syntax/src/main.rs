struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: Self) -> bool {
        self.height > other.height && self.width > other.width
    }
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 2,
    };
    let rect2 = Rectangle {
        width: 22,
        height: 2,
    };
    let rect3 = Rectangle {
        width: 32,
        height: 3,
    };
    let square1 = Rectangle::square(4);
    println!("{}", rect1.can_hold(rect2));
    println!("{}", rect1.can_hold(rect3));
    println!("{}", rect1.width());
    println!("{}", square1.area())
}
