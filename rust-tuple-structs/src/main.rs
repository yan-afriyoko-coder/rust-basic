struct Color(
    u8,
    u8
);

fn main() {
  
    let mut color = Color(8,9);
    color.0=10;

    println!("{}",color.0);

}
