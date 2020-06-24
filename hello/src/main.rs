mod fibonacci;
use crate::fibonacci::fibonacci;
fn main(){
    for i in 3..10{
        println!("{}",fibonacci(i));
    }
}


