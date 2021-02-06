use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub struct Cacher<T, U>
    where T: Fn(U) -> U {
    calculate: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U> where T: Fn(U) -> U, U: Clone {
    pub fn new(calculate: T) -> Cacher<T, U> {
        Cacher { calculate, value: None }
    }

    pub fn get_value(&mut self, arg: U) -> U {
        match self.value.clone() {
            None => {
                let v = (self.calculate)(arg);
                self.value = Some(v.clone());
                v
            }
            Some(v) => v
        }
    }
}