pub mod garden;
use garden::vegetables::Asparagus;
fn main() {
    let plant = Asparagus {};
    println!("{plant:?}");
}
