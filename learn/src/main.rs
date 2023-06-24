use crate::closure::Cacher;
use crate::generic::{Point, Summary};
use std::thread;
use std::time::Duration;

mod closure;
mod concurrent;
mod generic;
mod iter;

fn main() {
    let mut cache = Cacher::new({ |x| -> i32 { x * 2 } });
    let result = cache.get_value(3);
    println!("\nthe result is: {}", result);

    let mut cache2 = Cacher::new(|data| -> i64 {
        let result = data % 3;
        match result {
            0 => data * 10,
            1 => data / 2,
            _ => data * 3 + 4,
        }
    });

    let mut number = 1;
    while number < 100 {
        println!("\nthe result of cache2 is: {}", cache2.get_value(number));
        number += 1;
        cache2.clear()
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| -> u32 {
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
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
