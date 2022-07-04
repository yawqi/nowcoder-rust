use std::collections::{HashMap, HashSet};
fn main() {
    let mut map = HashMap::new();
    let commands = vec!["reset", "reset board", "board add", "board delete", "reboot backplane", "backplane abort"];
    let results = vec!["reset what", "board fault", "where to add", "no board at all", "impossible", "install first"];
    for i in 0..commands.len() {
        map.insert(commands[i], results[i]);
    }
    let first_part = vec!["reset", "board", "reboot", "backplane"];
    let second_part = vec!["board", "add", "delete", "backplane", "abort"];
    let mut first_map = HashMap::new();
    let mut first_set = HashSet::new();
    for first in first_part {
        for len in 1..=first.len() {
            if first_set.contains(&first[0..len]) {
                first_map.remove(&first[0..len]);
            } else {
                first_map.insert(&first[0..len], first);
                first_set.insert(&first[0..len]);
            }
        }
    }
    let mut second_map = HashMap::new();
    let mut second_set = HashSet::new();
    for second in second_part {
        for len in 1..=second.len() {
            if second_set.contains(&second[0..len]) {
                second_map.remove(&second[0..len]);
            } else {
                second_map.insert(&second[0..len], second);
                second_set.insert(&second[0..len]);
            }
        }
    }
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }
        let v = s.trim().split(" ").collect::<Vec<_>>();
        let first = match first_map.get(v[0]) {
            Some(word) => word,
            None => {
                println!("unknown command");
                continue;
            }
        };
        let second = if v.len() > 1 {
            match second_map.get(v[1]) {
                Some(word) => word,
                None => {
                    println!("unknown command");
                    continue;
                }
            }
        } else {
            ""
        };
        let mut s = first.to_string();
        if second.len() != 0 {
            s.push(' ');
            s.push_str(second);
        }
        match map.get(&s[..]) {
            Some(ret) => println!("{}", ret),
            None => println!("unknown command"),
        }
    }
}