fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);

    let mut v: Vec<_> = s.trim().chars().collect();
    v.sort();
    let s = v.into_iter().collect::<String>();
    println!("{}", s);
}