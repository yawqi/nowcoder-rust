fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let vs = s.trim().chars().collect::<Vec<_>>();
    let mut ret = vec![];
    let mut is_int = false;
    for ch in vs {
        if ch.is_digit(10) {
            if !is_int {
                is_int = true;
                ret.push('*');
            }
        } else {
            if is_int {
                is_int = false;
                ret.push('*');
            }
        }
        ret.push(ch);
    }
    if is_int {
        ret.push('*');
    }
    let s = ret.into_iter().collect::<String>();
    println!("{}", s);
}