use std::collections::BTreeMap;
use std::collections::HashSet;
fn main() {
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word).unwrap();
    let mut plain = String::new();
    std::io::stdin()
        .read_line(&mut plain).unwrap();
    let alphas = word.trim().chars().collect::<Vec<_>>();
    let mut occurs = BTreeMap::new();
    let mut choosen = HashSet::new();
    let mut count = 0;
    alphas.iter().for_each(|c| {
        if !choosen.contains(c) {
            occurs.insert(('a' as u8 + count as u8) as char, *c);
            choosen.insert(*c);
            count += 1;
        }
    });
    for i in 0..26 {
        let c = ('a' as u8 + i as u8) as char;
        if !choosen.contains(&c) {
            occurs.insert(('a' as u8 + count as u8) as char, c);
            choosen.insert(c);
            count += 1;
        }
    }
    let mut v: Vec<_> = plain.trim().chars().map(|c| occurs.get(&c).unwrap() ).collect();
    let s = v.into_iter().collect::<String>();
    println!("{}", s);
}