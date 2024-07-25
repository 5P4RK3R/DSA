// use std::io::{ BuffReader, BuffRead};
use std::io;
fn main() {
    let mut name:String = String::new();
    io::stdin().read_line(&mut name).expect("No Input Found");
    println!("Hello {}",name.trim_end())
}