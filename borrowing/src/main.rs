fn main() {
    let s1 = String::from("hello");
    let len = borrowing_calculate_length(&s1);
    println!("the length of string {s1} is {len}");
    mutable_ref();
}
fn borrowing_calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.
fn mutable_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}
