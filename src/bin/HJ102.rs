use std::{fs::read_link, collections::HashMap};

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let mut map = HashMap::new();
    s.trim().chars().for_each(|c| {
        let e = map.entry(c).or_insert(0);
        *e += 1;
    });
    let mut v = s.trim().chars().collect::<Vec<_>>();
    v.sort_by(|c1, c2| {
        let count1 = map.get(c1).unwrap();
        let count2 = map.get(c2).unwrap();
        if count1 != count2 {
            count2.cmp(count1)
        } else {
            c1.cmp(c2)
        }
    });
    v.dedup();
    println!("{}", v.into_iter().collect::<String>());
}