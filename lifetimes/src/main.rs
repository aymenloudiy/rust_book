fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

// need to specify lifetime, that way compiler knows return value has the same lifetime as the sorter lifetime of both params

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//annotation:
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// If a struct holds a reference we need to add a lifetime annotation

struct _ImportantExcerpt<'a> {
    part: &'a str,
}
// Still need to annotate lifetime here because the struct has a reference, even tho the method does not return or use the reference

impl<'a> _ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}
impl<'a> _ImportantExcerpt<'a> {
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn _main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i: _ImportantExcerpt<'_> = _ImportantExcerpt {
        part: first_sentence,
    };
}
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn _static_lifetime() {
    //this reference can live for the entire duration of the program
    let _s: &'static str = "I have a static lifetime.";
}

//eveything defined on a function: trait, generic and lifetime

use std::fmt::Display;

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
