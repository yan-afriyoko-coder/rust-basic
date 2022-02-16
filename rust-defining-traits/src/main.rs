
struct Person{
    name: String,
    age:u8
}

trait HasVoiceBox{

    fn speak(&self);

    fn can_speak(&self) ->bool;
}


impl HasVoiceBox for Person {

    fn speak(&self){
        println!("name {}", self.name);
    }

    fn can_speak(&self)-> bool{
         if self.age<= 30{
             return true;
         } return false;
    }
}
fn main(){

    let person = Person{
        name: String::from("Bob"),
        age:30
    };

    println!("can {}, speak? {}",person.name, person.can_speak())

    
}