fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s = s.trim().chars().collect::<Vec<_>>();
    for len in 0..s.len() {
        for start in 0..s.len()-len {
            if is_pad(&s[start..start+len]) {
                println!("{}", len);
                return;
            }
        }
    }
}

fn is_pad(s: &[char]) -> bool {
    s.iter().collect::<String>() == s.iter().rev().collect::<String>()
}