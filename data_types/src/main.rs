use std::io;
fn main() {
    let _eight_bit: i8 = 1;
    let _unsigned_eight_bit: u8 = 1;
    let _sixteen_bit: i16 = 1;
    let _unsigned_sixteen_bit: u16 = 1;
    let _thirty_two_bit: i32 = 1;
    let _unsigned_thirty_two_bit: u32 = 1_000;
    let _sixty_four_bit: i64 = 1;
    let _unsigned_sixty_four_bit: u64 = 1;
    let _one_hundread_twenty_eight_bit: i128 = 1;
    let _unsigned_one_hundread_twenty_eight_bit: u128 = 1;
    let _arch_bit: isize = 1;
    let _unsigned_arch_bit: usize = 1;
    let _sixty_four_float: f64 = 1.0;
    let _thrity_two_float: f32 = 1.0;
    invalid_element_access();
}
fn _numeric_operations() {
    let _sum = 1 + 1;
    let _difference = 1 - 1;
    let _product = 1 * 1;
    let _quotion = 1.2 / 1.5;
    let _remainder = 43 % 5;
}
fn _bool_type() {
    let _t = true;
    let _f: bool = false;
}
fn _char_type() {
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';
}
fn _compound_type() {
    let _tup: (i32, f64, u8) = (-12, 6.2, 7);
    let (x, y, z) = _tup;
    println!("x: {x}, y: {y},z:{z}");
    let _minus_twelve = _tup.0;
    let _six_point_2 = _tup.1;
    let _seven = _tup.2;
    let _a = [1, 2, 3, 4, 5];
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a = [3; 5];
    let _a = [1, 2, 3, 4, 5];
    let _first = _a[0];
    let _second = _a[1];
}
fn invalid_element_access() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index!");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please enter a number");
    let element = a[index];
    println!("Your element at the index {} is : {}", index, element);
}
