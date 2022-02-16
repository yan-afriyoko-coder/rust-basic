
fn main(){
    let mut dom = vec![1,2];

    dom.remove(0);
    dom.push(30);

    for elem in dom.iter() {
        println!("{}",elem);
    }
    
}