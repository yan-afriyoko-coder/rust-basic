fn main() {
    let x = 50;

    //code block
    {
        //isolated
        let y = 50;
        println!("Hello, world! {}{}",x,y);
    }
  
    // println!("Hello, world! {}{}",x,y); y akan error karena masuk kedalam code block

}
