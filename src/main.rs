//! Ho-Ho-Ho
#![allow(unused_imports)]
mod model;

use model::kids::Kid;

fn main() {
    let timmy = Kid::new(String::from("timmy"), 10 , 1);
    println!("Little {} is nice >> {}", timmy.name, timmy.is_nice());
}