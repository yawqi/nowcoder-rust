use std::collections::VecDeque;
fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let mut it = s.trim().split(' ');
    let n: usize= it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut items = vec![VecDeque::new(); m];
    for i in 0..m {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s);
        let mut iter = s.trim().split(' ');
        let cost: usize = iter.next().unwrap().parse().unwrap();
        let mut weight: usize = iter.next().unwrap().parse().unwrap();
        let master: usize = iter.next().unwrap().parse().unwrap();
        weight *= cost;
        if master == 0 {
            items[i].push_front((cost, weight));
        } else {
            items[master-1].push_back((cost, weight));
        }
    }
    
    let mut real_items = vec![];
    for item in items {
        if item.len() <= 1 {
            if item.len() == 1 {
                real_items.push(item.into_iter().collect());
            }
            continue;
        }
        let (c, w) = item[0];
        // let mut f = vec![w; n - c + 1];
        // let 
        // for (ci, wi) in item.into_iter().skip(1) {
        //     // println!("{}, {}", ci, wi);
        //     for v in (ci..=n-c).rev() {
        //         f[v] = std::cmp::max(f[v-ci]+wi, f[v]);
        //     }
        // }
        let mut real_item = vec![];
        real_item.push(item[0]);
        if item.len() >= 2 {
            real_item.push((c + item[1].0, w + item[1].1));
        }
        if item.len() == 3 {
            real_item.push((c + item[2].0, w + item[2].1));
            real_item.push((c + item[2].0 + item[1].0, w + item[2].1 + item[1].1));
        }
        // println!("real item: {:?}", real_item);
        real_items.push(real_item);
    }
    
    let mut result = vec![0; n+1];
    for item in real_items {
        for v in (0..=n).rev() {
            for (cost, weight) in &item {
                if v < *cost {
                    continue;
                }
                result[v] = std::cmp::max(result[v], result[v-cost] + weight);
            }
        }
    }
    println!("{}", result[n]);
}