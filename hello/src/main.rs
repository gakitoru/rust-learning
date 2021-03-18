fn main() {
    let s1: String = String::from("Hello World!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{0}, {1}, {2}", s1, s2, s3);
}
