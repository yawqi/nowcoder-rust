fn main() {
    let mut dna = String::new();
    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut dna).unwrap();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    let dna = dna.trim().chars().collect::<Vec<_>>();
    let n = n.trim().parse::<usize>().unwrap();
    let mut count = 0;
    let mut max_count = 0;
    for ch in &dna[0..n] {
        if *ch == 'G' || *ch == 'C' {
            count += 1;
        }
    }
    max_count = count;
    let mut res = &dna[0..n];
    for idx in n..dna.len() {
        if dna[idx] == 'G' || dna[idx] == 'C' {
            count += 1;
        }
        if dna[idx-n] == 'G' || dna[idx-n] == 'C' {
            count -= 1;
        }
        if max_count < count {
            max_count = count;
            res = &dna[idx-n+1..=idx];
        }
    }
    println!("{}", res.iter().collect::<String>());
}