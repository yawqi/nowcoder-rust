fn main() {
    let mut xyz = vec![];
    for _ in 0..3 {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s).unwrap();
        xyz.push(s.trim().parse::<usize>().unwrap());
    }
    let mut a = vec![];
    for _ in 0..xyz[0] {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line);
        a.push(
            line
                .trim()
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        );
    }
    let mut b = vec![];
    for _ in 0..xyz[1] {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line);
        b.push(
            line
                .trim()
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        );
    }

    for row in &a {
        for col_num in 0..xyz[2] {
            let mut sum = 0;
            for i in 0..row.len() {
                sum += row[i]*b[i][col_num];
            }
            print!("{} ", sum);
        }
        println!();
    }
}