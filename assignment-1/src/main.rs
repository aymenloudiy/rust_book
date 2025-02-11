fn main() {
    temperature_converter(32, 'c');
    temperature_converter(32, 'f');
    temperature_converter(32, 'k');
}
fn temperature_converter(temp: i32, unit: char) {
    if unit == 'C' || unit == 'c' {
        let temp_f = temp + 32;
        println!("The temperature in Fahrenheit is: {}", temp_f);
    } else if unit == 'F' || unit == 'f' {
        let temp_c = temp - 32;
        println!("The temperature in Celceius is: {}", temp_c);
    } else {
        println!("I don't know this unit.");
    }
}
