fn main() {
    let tup1 = (30,"animal",3.4,(1,2,3));
    let (a,b,c,(d,e,f)) = tup1;

    //println!("Tuples {}", tup1.0);
    //println!("sub Tuples {}", (tup1.3).2);

    println!("sub Tuples {}", f);
}
