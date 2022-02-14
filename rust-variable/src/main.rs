fn main(){
    
    let x = 5; // variabel tidak bisa diperbaharui
    let mut b = 5;  //variabel  bisa diperbaharui
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //konstata 

    println!("The variabel of x is: {}", x);
    println!("The variabel of x is: {}", b);
    println!("The Konstata of x is: {}", THREE_HOURS_IN_SECONDS);

    b=3;
    // x =4; "perlu menambahakan "mut" agar bisa berubah"

    println!("The variabel of x is: {}", b);
}