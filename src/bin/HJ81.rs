use std::collections::{HashMap, HashSet};

fn main() {
    loop {
        let mut short = String::new();
        let mut long = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut short).unwrap();
        if bytes == 0 {
            break;
        }
        std::io::stdin()
            .read_line(&mut long);
        
        let mut hset = HashSet::new();
        long.chars()
            .for_each(|c| {
                hset.insert(c);
            });
        let ret = short.chars()
                        .all(|c| hset.contains(&c));
        println!("{}", ret);
    }
}