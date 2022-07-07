use std::{collections::HashSet, hash::Hash};

fn main() {
    let mut n = String::new();
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut nums);
    let n = n.trim().parse::<usize>().unwrap();
    let nums = nums.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut fives = 0;
    let mut threes = 0;
    let mut others = vec![];
    let mut neg_sum = 0;
    let mut pos_sum = 0;
    let mut set = HashSet::new();
    for n in nums {
        match n {
            n if n % 5 == 0 => fives += n,
            n if n % 3 == 0 => threes += n,
            n => {
                others.push(n);
                if n < 0 {
                    neg_sum += n;
                } else {
                    pos_sum += n;
                }

                if set.is_empty() {
                    set.insert(n);
                } else {
                    let mut new_set = HashSet::new();
                    for v in &set {
                        new_set.insert(v - n);
                        new_set.insert(v + n);
                    }
                    set = new_set;
                }
                others.push(n);
            },
        }
    }

    if (fives - threes).abs() > pos_sum - neg_sum {
        println!("false");
        return;
    }

    if set.contains(&(fives - threes)) || set.contains(&-(fives - threes)) || fives - threes == 0 {
        println!("true");
        return;
    }
    

    println!("false");
}