fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s = s.trim().chars().rev().collect::<String>();
    println!("{}", s);
}