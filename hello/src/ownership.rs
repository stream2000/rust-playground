/**
 * @Author : Fred Fu
 * @Time: 2020/6/24 12:00 
 */


pub fn move_main() {
    ya_mut();
}

// understand the difference between borrow and ownership changing and mutable borrow

fn ref_str(str: &String) {
    println!("{}", str)
}

fn mut_srt(s: &mut String) {
    s.push_str(" mut")
}

fn ya_mut() {
    let mut s = String::from("hello");

    // it's reasonable
    {
        let r1 = &s;
        println!("{}", r1);
    }
    // let r2 = &mut s;

    s.push_str("hello");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}