fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin()
        .read_line(&mut s1);
    std::io::stdin()
        .read_line(&mut s2);
    
    let v1 = s1.trim().chars().collect::<Vec<_>>();
    let v2 = s2.trim().chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; v2.len() + 1]; v1.len() + 1];
    let mut max = 0;
    for i in 1..=v1.len() {
        let c1 = v1[i-1];
        for j in 1..=v2.len() {
            let c2 = v2[j-1];
            if c1 == c2 {
                dp[i][j] = dp[i-1][j-1] + 1;
                max = std::cmp::max(max, dp[i][j]);
            } else {
                dp[i][j] = 0;
            }
        }
    }
    println!("{}", max);
}