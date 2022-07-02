fn main() {
    loop {
        let mut n = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut n).unwrap();
        if bytes == 0 {
            break;
        }
        let n = n.trim().parse::<usize>();
        let mut nums = String::new();
        std::io::stdin()
            .read_line(&mut nums).unwrap();
        let nums = nums.trim()
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
    
        let mut rank = String::new();
        std::io::stdin()
            .read_line(&mut rank).unwrap();
    
        let rank = rank.trim().parse::<usize>().unwrap();
        let ret = nums.into_iter().nth_back(rank-1).unwrap();
        println!("{}", ret);
    }
}