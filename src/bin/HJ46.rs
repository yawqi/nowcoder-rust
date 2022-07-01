fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    println!("{}", &s[..n]);
}