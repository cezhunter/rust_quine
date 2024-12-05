fn main() {
    let mut s = String::new();
    s.push(char::from_u32(34).unwrap());
    s.push_str(
        "
    fn main() {{
        let mut s = String::new();
        s.push(char::from_u32(34).unwrap());
        s.push_str({0},);
        s.push(char::from_u32(34).unwrap());
        println!({0}, s);
    }}",
    );
    s.push(char::from_u32(34).unwrap());
    println!(
        "
    fn main() {{
        let mut s = String::new();
        s.push(char::from_u32(34).unwrap());
        s.push_str({0},);
        s.push(char::from_u32(34).unwrap());
        println!({0}, s);
    }}",
        s
    );
}
