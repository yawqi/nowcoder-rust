use std::collections::HashMap;

fn main() {
    let mut n = String::new();
    let mut res = vec![];
    std::io::stdin()
        .read_line(&mut n).unwrap();

    let n = n.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        let mut s = String::new();
        let mut count_map = HashMap::new();
        std::io::stdin()
            .read_line(&mut s).unwrap();
        let mut s = s.trim().chars().collect::<Vec<_>>();
        s.iter()
        .for_each(|c| {
            let e = count_map.entry(*c).or_insert(0);
            *e += 1;
        });

        s.sort_by_key(|c| -count_map.get(c).unwrap());
        let mut mark_map = HashMap::new();
        let mut count = 26;
        for ch in &s {
            if !mark_map.contains_key(ch) {
                mark_map.insert(*ch, count);
                count -= 1;
            }
        }
        let mut sum = 0;
        s.into_iter().for_each(|c| sum += *mark_map.get(&c).unwrap());
        res.push(sum);
    }
    for s in res {
        println!("{}", s);
    }
}