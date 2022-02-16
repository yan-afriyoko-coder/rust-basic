struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn print_description(&self){
        println!("Rectangle: {} X {}", self.width,self.height)
    }

    fn is_squale(&self)->bool {
        self.width==self.height
    }
}

fn main() {
    let cetak =  Rectangle{width:20,height:30};
    cetak.print_description();
    println!("bool: {}", cetak.is_squale());
}
