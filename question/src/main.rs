fn main() {
    let t = ("hello".to_string(), "world".to_string());
    let (a, _) = t;
    println!("{} {}", a, t.0);
}
