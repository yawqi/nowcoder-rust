fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);

    let n = s.trim().parse::<usize>().unwrap();
    let mut vs: Vec<Vec<String>> = vec![vec![]; n];

    let mut start = 1;
    let mut count = n;
    for i in 0..n {
        vs[i].push(start.to_string());
        let mut prev = vs[i][0].parse::<usize>().unwrap();
        for c in 1..count {
            vs[i].push((prev+i+c+1).to_string());
            prev += i+c+1;
        }
        count -= 1;
        start += i + 1;
    }

    for v in vs {
        println!("{}", v.join(" "));
    }
}