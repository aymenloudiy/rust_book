use std::{collections::HashMap, io};

fn main() {
    let list_test = vec![4, 3, 4, 2, 1, 0];
    let sorted_list = sort_list(list_test);
    let median = get_median(&sorted_list);
    let mode = get_mode(&sorted_list);
    println!("{median}");
    println!("{mode}");
    let string_test = String::from("Hello World");
    let pigged = pig_latin(string_test);
    println!("{pigged}");
    let mut interface_map: HashMap<String, Vec<String>> = HashMap::new();
    interface_map.insert(String::from("Engineering"), vec![String::from("Aymen")]);
    interface_map = interface(interface_map);
    println!("{interface_map:?}")
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
fn interface(mut company: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut instruction: String = String::new();
    io::stdin()
        .read_line(&mut instruction)
        .expect("Failed to read line");

    let instruction_vec: Vec<&str> = instruction.split_whitespace().collect();
    if instruction_vec.len() != 4 {
        println!("The instruction should be in the format of : add x to y, if there is a name and surname use a hyphen to separate them");
        return company;
    }
    let department = instruction_vec[3].to_string();
    let name = instruction_vec[1].to_string();
    match instruction_vec[0].to_lowercase().as_str() {
        "add" => {
            println!("Added {},to {}", department, name);
            let people = company.entry(department).or_insert(vec![]);
            people.push(name);
            people.sort();
        }
        _ => {
            println!("The only possible action is 'add' , the format should be 'Add x to y' ");
        }
    };
    company
}
