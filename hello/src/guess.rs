/**
 * @Author : Fred Fu
 * @Time: 2020/6/24 11:27 
 */
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::empty;

fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // so the `expect` here is used for error handling
        // let guess: u32 = guess.trim().parse()
        //     .expect("You should input a number");

        // pattern match　は最高だ
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
