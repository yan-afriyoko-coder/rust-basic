struct Color {
    red:u8,
    green:u8
}

fn main() {
    // let warna = Color{red: 8,green:9};
    let mut warna_dinamc = Color{red: 8,green:9};
    warna_dinamc.red=10;

    // println!("{}",warna.red);
    println!("{}",warna_dinamc.red);

}
