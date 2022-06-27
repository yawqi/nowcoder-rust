fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s: String = s.trim().chars().map(|c| if c.is_ascii_alphabetic() {
        c
    } else {
        ' '
    }).collect();

    let v: Vec<_> = s.split(" ").collect();
    let v: Vec<_> = v.into_iter().rev().collect();
    let s = v.join(" ");
    println!("{}", s);
}