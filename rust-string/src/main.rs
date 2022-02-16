fn main() {
    // Manipulasi String
    let mut my_string = String::from("halooo yan");
    println!("length {}",my_string.len());

    for token in my_string.split_whitespace(){
        println!("{}", token);
    }

    println!("Do {}",my_string.contains("yan"));
    my_string.push_str("wellcome");
    println!("{}",my_string);

}
