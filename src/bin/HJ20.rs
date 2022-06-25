use std::collections::HashSet;

fn main() {
    let mut v = vec![];
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }
        let s: Vec<_>= s.trim().chars().collect();
        if s.len() < 8 {
            v.push(false);
            continue;
        }

        let mut conditions = 0;
        let mut digit = false;
        let mut upper = false;
        let mut lower = false;
        let mut others = false;
        let mut map = HashSet::new();
        let mut contains = false;
        s[..].windows(3)
            .for_each(|w| {
                if map.contains(w) {
                    contains = true;
                } else {
                    map.insert(w);
                }
            });
        if contains {
            v.push(false);
            continue;
        }
        s.into_iter()
            .for_each(|c| {
                if c.is_digit(10) {
                    if !digit {
                        digit = true;
                        conditions += 1;
                    }
                } else if c.is_uppercase() {
                    if !upper {
                        upper = true;
                        conditions += 1;
                    }
                } else if c.is_lowercase() {
                    if !lower {
                        lower = true;
                        conditions += 1;
                    }
                } else {
                    if !others {
                        others = true;
                        conditions += 1;
                    }
                }
            });
        if conditions >= 3 {
            v.push(true);
        } else {
            v.push(false);
        }

    }

    for r in v {
        if r {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}