fn largest<T>(list: &[T]) -> &T where T: PartialOrd {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::{Display, Debug};


pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Point<i32> {
    fn summary(&self) -> String {
        format!("x : {}; y : {}", &self.x, &self.y)
    }
}

const s: &'static str = "hello";

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summary());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

pub fn longer_string<'a, T>(a: &'a T, b: &'a T) -> &'a T where T: PartialOrd {
    if a > b {
        a
    } else {
        b
    }
}

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }

