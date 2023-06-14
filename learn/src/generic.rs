use std::fmt::{Debug, Display};
use std::ops::Add;

fn largest<T>(list: &[T]) -> &T
    where
        T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Don't you think it's cool?
struct Tuple3(i32, i32, i32);


#[derive(Debug, Copy, Clone, PartialEq)]
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

impl<T: Add + Copy + Add<Output=T>> Point<T> {
    pub fn add(&mut self, other: &Point<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
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

const HELLO_STR: &'static str = "hello";

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summary());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    1
}

pub fn longer_string<'a, T>(a: &'a T, b: &'a T) -> &'a T
    where
        T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}


#[cfg(test)]
mod test {
    use crate::generic::{notify, Point};

    #[test]
    fn basics() {
        let mut p = Point::new(1, 2);
        p.add(&Point::new(3, 4));
        notify(&p);
    }
}
