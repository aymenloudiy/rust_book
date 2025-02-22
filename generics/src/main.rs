struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self.x
    }
}
// This method only works on isntances of Point with type f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let number_list = vec![1, 2, 3, 4, 5, 6];
    let result = largest_i32(&number_list);
    println!("{result}");
    let char_list = vec!['q', 'w', 'e', 'r', 't', 'y', 'u'];
    let result = largest_char(&char_list);
    println!("{result}");
    let result = largest_type(&number_list);
    println!("{result}");
    let result = largest_type(&char_list);
    println!("{result}");
    let _integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let dist = float.distance_from_origin();
    println!("{dist}")
}
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_type<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
