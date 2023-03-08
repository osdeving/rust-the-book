#![allow(unused, dead_code)]

use std::cmp::max;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue= 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue = num_blue + 1,
                ShirtColor::Red => num_red = num_red + 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let shirts = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref = Some(ShirtColor::Red);
    let giveaway = shirts.giveaway(user_pref);

    println!("The user with preference {:?} get {:?}", user_pref, giveaway);

    let user_pref = None;
    let giveaway = shirts.giveaway(user_pref);

    println!("The user with preference {:?} get {:?}", user_pref, giveaway);

}

