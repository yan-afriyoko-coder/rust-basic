
use std::fs::File;
use std::io::prelude::*;

fn main(){
 let mut file = File::open("text.txt").expect("tidak bisa buka filenya");
 let mut contents = String::new();
 file.read_to_string(&mut contents)
 .expect("Oops");

 println!("{}",contents);
    
}