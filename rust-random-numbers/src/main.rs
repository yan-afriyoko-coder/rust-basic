extern crate rand;
use Rand::Rng;

fn main(){
    let random_number =rand::thread_rng().gen_range(1,11); // 1-10
    println!("Random Nul {}",random_number);
    
}