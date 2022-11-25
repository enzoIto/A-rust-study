fn main() {
    let s = String::from("Hello World");
    let (_, s1) = &s.rsplit_once(' ').unwrap();
    println!("{}", &s1);
    
}
