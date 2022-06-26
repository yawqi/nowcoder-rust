use std::{ascii::AsciiExt, cmp::Ordering};

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s = s.trim().to_string();
    let len = s.len();
    let mut ret = vec![' '; len];
    let mut pos = vec![];
    let mut chs = vec![];
    let mut order = [0; 26];
    s.chars()
        .enumerate()
        .for_each(|(i, c)| {
            if c.is_alphabetic() {
                chs.push(c);
                // if c.is_uppercase() && order[c as usize - 'A' as usize] == 0 {
                //     order[c as usize - 'A' as usize] = 2;
                // } else if c.is_lowercase() && order[c as usize - 'a' as usize] == 0 {
                //     order[c as usize - 'a' as usize] = 1;
                // }
                pos.push(i);
            } else {
                ret[i] = c;
            }
        });
    chs.sort_by(|t1, t2| {
        // if t1.to_ascii_lowercase() == t2.to_ascii_lowercase() && t1 != t2 {
        //     if order[t1.to_ascii_lowercase() as usize - 'a' as usize] == 1 {
        //         if t1.is_lowercase() {
        //             return Ordering::Less;
        //         }
        //         return Ordering::Greater;  
        //     } 
        //     if t1.is_lowercase() {
        //         return Ordering::Greater;
        //     }
        //     return Ordering::Less; 
        // }
        t1.to_ascii_lowercase().cmp(&t2.to_ascii_lowercase())
    });
    let mut count = 0;
    for i in pos {
        ret[i] = chs[count];
        count += 1;
    }
    println!("{}", ret.into_iter().collect::<String>());
}