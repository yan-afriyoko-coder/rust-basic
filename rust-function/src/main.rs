fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 //mengembalikan nilai tanpa titik koma
    };
    let five = five();
    println!("The value of y is: {}", y);
    println!("The five of y is: {}", five);
}

fn another_function(x: i32) {
    println!("Another function{}. ", x);
  
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}
