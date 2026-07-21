fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function.");
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
