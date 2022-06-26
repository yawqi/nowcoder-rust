fn main() {
    let mut n_str = String::new();
    std::io::stdin()
        .read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();
    let mut students_str = String::new();
    std::io::stdin()
        .read_line(&mut students_str).unwrap();
    let students: Vec<_> = students_str.trim()
                    .split(' ')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
    let mut l2r = vec![1; n];
    let mut r2l = vec![1; n];
    let mut ret = 0;
    students.iter()
        .enumerate()
        .for_each(|(i, height)| {
            for index in 0..i {
                if *height > students[index] {
                    l2r[i] = std::cmp::max(l2r[i], l2r[index] + 1);
                }
            }
        });
    
    students.iter()
        .enumerate()
        .rev()
        .for_each(|(i, height)| {
            for index in i+1..n {
                if *height > students[index] {
                        r2l[i] = std::cmp::max(r2l[i], r2l[index] + 1);
                }
            }
        });
    for (l, r) in l2r.iter().zip(r2l.iter()) {
        ret = std::cmp::max(ret, *l + *r - 1);
    }
    println!("{}", n - ret);
}