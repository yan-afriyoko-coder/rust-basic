
use std::fs::File;
use std::io::prelude::*;

fn main(){
 let mut file = File::create("outpu.txt").expect("tidak bisa buka filenya");
 file.write_all("halo gan").expect("oops")

    
}