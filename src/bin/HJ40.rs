fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let mut alphas = 0;
    let mut spaces = 0;
    let mut digits = 0;
    let mut others = 0;
    s.trim().chars()
        .for_each(|c| {
            if c.is_alphabetic() {
                alphas += 1;
            } else if c.is_whitespace() {
                spaces += 1;
            } else if c.is_digit(10) {
                digits += 1;
            } else {
                others += 1;
            }
        });
    println!("{}", alphas);
    println!("{}", spaces);
    println!("{}", digits);
    println!("{}", others);
}