fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3, 4];
    let mut _v3: Vec<i32> = Vec::new();
    _v3.push(1);
    _v3.push(2);
    _v3.push(3);
    _v3.push(4);
    read_vec_elements();
    iterate_vector_values();
    iterate_vector_change();
    enum_vector();
}
fn read_vec_elements() {
    let v = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &v[2];
    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth item is {fourth}"),
        None => println!("There is no fourth element"),
    }
}
fn iterate_vector_values() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}")
    }
}
fn iterate_vector_change() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
}
fn enum_vector() {
    //used to store mutliple types in a vector
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _v: Vec<Spreadsheet> = vec![
        Spreadsheet::Int(5),
        Spreadsheet::Float(0.5),
        Spreadsheet::Text(String::from("Hello")),
    ];
}
