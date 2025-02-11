fn main() {
    println!("Hello, world!");

    print_label_measurment(5, 'm');
    println!("{}", plus_one(2));
    println!("{}", plus_two(2));
}
fn print_label_measurment(x: i32, unit_label: char) {
    println!("The measurment is {}{}", x, unit_label);
}
fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression
}
fn plus_two(y: i32) -> i32 {
    let z = 2; // this is a statment
    return y + z; // this is also an expression
}
