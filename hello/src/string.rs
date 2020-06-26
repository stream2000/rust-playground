use std::collections::HashMap;

/**
 * @Author : Fred Fu
 * @Time: 2020/6/26 12:24 
 */


pub fn str() {
    let data = "initial contents";

    let mut s = data.to_string();
    let s = &mut s;

    let right_str = String::from(" world");
    s.push_str(&right_str);
    println!("str: {}", s);
// 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let test_str = String::from(" hey");

    // add is called with ownership changing, so we need a new variable to get the ownership back
    let test_str = test_str + &s;

}