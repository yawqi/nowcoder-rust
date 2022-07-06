fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let mut count = 0;
    s.trim()
        .chars()
        .for_each(|c| if c.is_ascii_uppercase() { count += 1; });
    println!("{}", count);
}