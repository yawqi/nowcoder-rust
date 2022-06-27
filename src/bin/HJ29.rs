use std::ascii::AsciiExt;

fn main() {
    let mut plain = String::new();
    let mut cipher = String::new();
    std::io::stdin()
        .read_line(&mut plain);
    std::io::stdin()
        .read_line(&mut cipher);
    println!("{}", crypt(plain.trim(), 1));
    println!("{}", crypt(cipher.trim(), -1));
}

fn crypt(s: &str, shift: i8) -> String {
    s.chars()
        .map(|c| {
            if c.is_digit(10) {
                (('0' as i8 + (c as i8 - '0' as i8  + shift as i8 + 10) % 10 as i8) as u8) as char
            } else if c.is_ascii_uppercase() {
                (('a' as i8 + (c as i8 - 'A' as i8  + shift as i8 + 26) % 26 as i8) as u8) as char
            } else if c.is_ascii_lowercase() {
                (('A' as i8 + (c as i8 - 'a' as i8  + shift as i8 + 26) % 26 as i8) as u8) as char
            } else {
                c
            }
        })
        .collect()
}