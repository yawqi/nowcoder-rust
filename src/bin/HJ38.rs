fn main() {
    let mut height = String::new();
    std::io::stdin()
        .read_line(&mut height).unwrap();
    let height: f64 = height.trim().parse().unwrap();
    println!("{}", height * 23.0 / 8.0);
    println!("{}", height / 32.0);
}