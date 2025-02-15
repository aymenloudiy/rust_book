struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    debugged_main();
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// With debug
#[derive(Debug)]
struct DebugedRectangle {
    width: u32,
    height: u32,
}

fn debugged_main() {
    let scale = 2;
    let rect1 = DebugedRectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("{rect1:#?},{},{}", rect1.width, rect1.height);
    dbg!(&rect1);
}
