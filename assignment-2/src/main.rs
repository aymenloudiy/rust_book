use std::collections::HashMap;

fn main() {
    let list_test = vec![4, 3, 4, 2, 1, 0];
    let sorted_list = sort_list(list_test);
    let median = get_median(&sorted_list);
    let mode = get_mode(&sorted_list);
    println!("{median}");
    println!("{mode}");
    let string_test = String::from("Hello World");
    let pigged = pig_latin(string_test);
    println!("{pigged}")
}
fn sort_list(list: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = list;
    answer.sort();
    answer
}
fn get_median(list: &Vec<i32>) -> i32 {
    list[list.len() / 2]
}
fn get_mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in list {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut final_answer: i32 = 0;
    for (_, value) in map {
        if value > final_answer {
            final_answer = value;
        }
    }
    final_answer
}
fn pig_latin(s: String) -> String {
    let mut answer = String::from("");
    let mut pig: String;
    let vowels = vec!["a", "o", "u", "i", "e"];
    for word in s.split_whitespace() {
        let (first, last) = word.split_at(1);
        if vowels.contains(&first) {
            pig = format!("{last}-{first}hay");
        } else {
            pig = format!("{last}-{first}ay");
        }
        answer = format!("{answer} {pig}");
    }
    answer.trim().to_owned()
}
