fn main() {
    let s1 = String::from("hello");

    calculate_length(&s1);

}

fn calculate_length(s: &String){
    println!("s = {}", s);
}
