use crate::generic::{Summary, Point};
use std::thread;
use std::time::Duration;
use crate::closure::Cacher;

mod generic;
mod closure;
mod iter;

fn main() {
    let mut cache = Cacher::new({ |x: i32| -> i32  { x * 2 } });
    let result = cache.get_value(3);
    println!("result {}", result);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

