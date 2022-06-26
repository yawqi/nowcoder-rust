use std::{usize, collections::HashMap, collections::BTreeSet};

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let v = s.trim()
            .split(' ')
            .collect::<Vec<_>>();
    let n = v[0].parse::<usize>().unwrap();
    let m = v.last().unwrap().parse::<usize>().unwrap();
    let word = v[n+1].to_string();
    let mut word_map = HashMap::new();
    let mut words = vec![];
    word.chars()
        .for_each(|ch| {
            let e = word_map.entry(ch).or_insert(0);
            *e += 1;
        });
    for w in &v[1..=n] {
        if w.len() == word.len() && *w != word {
            let mut m = HashMap::new();
            w.chars()
                .for_each(|c| {
                    let e = m.entry(c).or_insert(0);
                    *e += 1;
                });
            if m == word_map {
                words.push(w.to_string());
            }
        }
    }
    words.sort();
    println!("{}", words.len());
    println!("{:?}", words);
    if words.len() >= m {
        println!("{}", words[m-1]);
    }
}