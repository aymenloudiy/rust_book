#[derive(Debug, Clone, Copy, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => blue += 1,
                ShirtColor::Red => red += 1,
            }
        }
        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref1 = ShirtColor::Blue;
    let user_pref2 = ShirtColor::Red;
    let give_away1 = store.giveaway(Some(user_pref1));
    let give_away2 = store.giveaway(Some(user_pref2));
    println!("Person1 gets a {:?} shirt", give_away1);
    println!("Person2 gets a {:?} shirt", give_away2);
}
