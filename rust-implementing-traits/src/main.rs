struct Person {
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self)-> String{
        return format!("My Name Is {}", self.age)
    }
}
fn main(){
    let dom = Person {name: String::from("Yan"),age:21};
    println!("{}",dom.name)
}