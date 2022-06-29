use std::{collections::HashSet, hash::Hash};
fn main() {
    let mut n = String::new();
    let mut weights = String::new();
    let mut counts = String::new();

    std::io::stdin()
        .read_line(&mut n).unwrap();
    std::io::stdin()
        .read_line(&mut weights).unwrap();
    std::io::stdin()
        .read_line(&mut counts).unwrap();

    let n = n.trim().parse::<usize>().unwrap();
    let weights: Vec<_> = weights
                            .trim()
                            .split(" ")
                            .map(|w| w.parse::<u32>().unwrap())
                            .collect();
    let counts: Vec<_> = counts
                            .trim()
                            .split(" ")
                            .map(|c| c.parse::<u32>().unwrap())
                            .collect();
    let mut set: HashSet<u32> = HashSet::new();
    let mut vs: Vec<u32> = Vec::new();
    for i in 0..n {
        // let v = (0..).step_by(weights[i] as usize).take(counts[i] as usize +1).collect::<Vec<_>>();
        vs.extend(std::iter::repeat(weights[i]).take(counts[i] as usize).collect::<Vec<_>>());
    }
    set.insert(0);
    for num in vs {
        let s = set.clone();
        s.into_iter().for_each(|v| {set.insert(v + num);});
    }
    // accumate(0, 0, &vs, &mut set);
    println!("{}", set.len());
}

// fn accumate(curr: u32, index: usize, vs: &Vec<Vec<u32>>, set: &mut HashSet<u32>) {
//     for num in &vs[index] {
//         if index == vs.len() - 1 {
//             set.insert(curr + *num);
//         } else {
//             accumate(curr + *num, index + 1, vs, set);
//         }
//     }
// }