fn main() {
    let mut x = 52;
    let dom = &mut x;
    *dom+=1;


    println!("hasil {}",x );
    // println!("Hello, world! {}{}",x,y); y akan error karena masuk kedalam code block

}
