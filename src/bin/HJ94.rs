use std::collections::HashMap;

fn main() {
    let mut n = String::new();
    let mut names = String::new();
    let mut m = String::new();
    let mut tickets = String::new();

    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut names);
    std::io::stdin()
        .read_line(&mut m);
    std::io::stdin()
        .read_line(&mut tickets);
    
    let mut names = names.trim()
        .split(" ")
        .collect::<Vec<_>>();
    names.push("Invalid");
    let mut map = HashMap::new();
    names.iter()
        .for_each(|n| {map.insert(n, 0);});
    tickets.trim()
            .split(" ")
            .for_each(|n| if map.contains_key(&n) {
                *map.get_mut(&n).unwrap() += 1;
            } else {
                *map.get_mut(&"Invalid").unwrap() += 1;
            });
    for name in &names {
        println!("{} : {}", name, map.get(name).unwrap());
    }
}