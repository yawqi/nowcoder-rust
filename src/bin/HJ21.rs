use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s: String = s.trim()
        .chars()
        .map(|c| {
            if c.is_digit(10) {
                return c;
            }
            if c.is_lowercase() {
                match c {
                    'a' | 'b' | 'c' => '2',
                    'd' | 'e' | 'f' => '3',
                    'g' | 'h' | 'i' => '4',
                    'j' | 'k' | 'l' => '5',
                    'm' | 'n' | 'o' => '6',
                    'p' | 'q' | 'r' | 's' => '7',
                    't' | 'u' | 'v' => '8',
                    'w' | 'x' | 'y' | 'z' => '9',
                    _ => '1',
                }
            } else {
                ('a' as u8 + (c as u8 - 'A' as u8 + 1) % 26) as char
            }
        })
        .collect();

    println!("{}", s);
}