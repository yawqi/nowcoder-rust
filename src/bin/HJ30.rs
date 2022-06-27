fn main() {
    let mut s1 = String::new();
    std::io::stdin()
        .read_line(&mut s1).unwrap();
    let ss: Vec<_> = s1.trim().split(" ").collect();
    let s: Vec<_> = ss[0].chars().chain(ss[1].chars()).collect();
    let mut v1: Vec<_>= s.iter()
        .step_by(2)
        .collect();
    let mut v2: Vec<_>= s.iter()
        .skip(1)
        .step_by(2)
        .collect();

    v1.sort();
    v2.sort();
    let mut v = vec![' '; s.len()];
    for i in 0..v.len() {
        if i % 2 == 0 {
            v[i] = *v1[i/2];
        } else {
            v[i] = *v2[i/2];
        }
    }

    let ret: String = v.iter()
        .map(|c| {
            if c.is_digit(16) {
                let num = c.to_digit(16).unwrap();
                let s = format!("{:04b}", num);
                let rev: String = s.chars().rev().collect();
                // println!("{} {}", s, rev);
                let num: u32 = u32::from_str_radix(&rev, 2).unwrap();
                // println!("{}", num);
                char::from_digit(num, 16).unwrap().to_ascii_uppercase()
            } else {
                *c
            }
        }).collect();
    print!("{}", ret);
}