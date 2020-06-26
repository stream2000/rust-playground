mod functional;
mod string;
mod ownership;
mod fibonacci;
mod may;

use crate::may::dis;

use crate::ownership::move_main;

// it just like swift and other pls, right?
fn main() {
    // for i in 0..10{
    //     println!("{}",fibonacci(i));
    // }
    move_main();
    let rect = Rect { height: 4, width: 2 };
    println!("the area is {}", rect.area());
    println!("the square is {}", Rect::square(2));
    let c = dis::add(1, 2);
    // make sure you understand the type system
    // why don't the university teach something about programing language?
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // 老c-style了
    let maybe_null: Option<i32> = None;
    let x = match maybe_null {
        None => 3,
        Some(i) => {
            println!("i: {}", i);
            1
        }
    };
    println!("{}", x);

    if let Some(k) = maybe_null {
        println!("k: {}", k)
    } else {
        println!("null!")
    }

    let mut v = vec![1, 2, 3];
    v.push(2);

    let v2 = &v;
    println!("{:?}", v);
    println!("{:?}", v2);

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
    string::str();
    functional::main();
}

#[derive(Debug)]
struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(i: u32) -> u32 {
        i * i
    }
}