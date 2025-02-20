fn main() {
    // panic!("crash and burn");
    out_of_bound();
}
fn out_of_bound() {
    let v = vec![1, 2, 3];

    v[99];
}
