fn main() {
    let fah = 100;
    let cel = 32;

    println!("{}F = {}C", fah, to_celsius(fah));
    println!("{}C = {}F", cel, to_fahrenheit(cel))
}

fn to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}
