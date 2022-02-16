struct Color {
    red:u8,
    green:u8
}

fn main() {
    // let warna = Color{red: 8,green:9};
    let warna_dinamc = Color{red: 8,green:9};
    print_color(&warna_dinamc);
    print_color(&warna_dinamc);

}

fn print_color(c: &Color){
    println!("color red:{} red:{}", c.red,c.green);
}
