fn main() {
    let mut string = String::from("Hello, ");
    string.push_str("world!");
    func(&mut string);
    println!("{}", string);
}
fn func(s: &mut String) {
    s.push_str("!");
}
