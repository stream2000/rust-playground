use std::collections::HashMap;

/**
 * @Author : Fred Fu
 * @Time: 2020/6/26 13:27 
 */

pub fn main() {
    let publisher1 = vec![String::from("hello"),String::from("hello2")];
    let publisher2 = vec![1, 3, 4, 5, 6];

    let result: HashMap<_, _> = publisher1.iter().zip(publisher2.iter()).map(
        |tuple: (&String, &i32)| {
            let (key, value) = tuple;
            (format!("{} world",key), value + 1)
        }
    ).collect();

    println!("result: {:?}", result);
}