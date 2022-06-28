fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();

    let v: Vec<_> = s.trim().chars().collect();
    let mut odds_len = vec![1; v.len()];
    for i in 1..v.len()-1 {
        let mut l = i - 1;
        let mut r = i + 1;
        while r < v.len() && v[l] == v[r] {
            odds_len[i] += 2;
            if l == 0 {
                break;
            }
            l -= 1;
            r += 1;
        }
    }
    let mut evens_len = vec![0; v.len()];
    for i in 0..v.len()-1 {
        let mut l = i;
        let mut r = i + 1;
        while r < v.len() && v[l] == v[r] {
            evens_len[i] += 2;
            if l == 0 {
                break;
            }
            l -= 1;
            r += 1;
        }
    }
    let mut max = 1;
    odds_len.into_iter().for_each(|i| max = std::cmp::max(max, i));
    evens_len.into_iter().for_each(|i| max = std::cmp::max(max, i));
    println!("{}", max);
}