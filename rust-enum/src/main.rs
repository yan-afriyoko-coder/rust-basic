enum Animals {
    name,
    size,
}

fn main() {
    let animals:Animals = Animals::name;
  
    match animals{
        Animals::name =>  println!("Gajah"),
        Animals::size =>  println!("30")
    }
}
