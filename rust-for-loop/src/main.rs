fn main() {
    let animals = vec!["kuda","kerbau"];
    
   for (index,elem) in animals.iter().enumerate() {
        println!("animals {} {}!",elem,index);
   }
}
