fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin()
        .read_line(&mut s1);
    std::io::stdin()
        .read_line(&mut s2);
    let mut s1 = s1.trim().chars().collect::<String>();
    let mut s2 = s2.trim().chars().collect::<String>();
    if s1.len() > s2.len() {
        std::mem::swap(&mut s1, &mut s2);
    }

    let mut max_len = 0;
    let mut max_str = &s1[..];
    // for i in 0..s1.len() {
    //     if s1.len() - i + 1 <= max_len {
    //         break;
    //     }
    //     for j in (i..s1.len()).rev() {
    //         let sub_str = &s1[i..=j];
    //         if s2.contains(sub_str) {
    //             if sub_str.len() > max_len {
    //                 max_len = sub_str.len();
    //                 max_str = sub_str;
    //             }
    //         }
    //     }
    // }
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 1..=s1.len() {
        let c1 = s1.chars().nth(i-1).unwrap();
        for j in 1..=s2.len() {
            let c2 = s2.chars().nth(j-1).unwrap();
            if c1 == c2 {
                dp[i][j] = dp[i-1][j-1] + 1;
                if dp[i][j] > max_len {
                    max_len = dp[i][j];
                    max_str = &s1[i-max_len..i]
                }
            }
        }
    }
    println!("{}", max_str);
}