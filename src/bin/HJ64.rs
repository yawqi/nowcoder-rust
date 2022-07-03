fn main() {
    let mut n = String::new();
    let mut ops = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    std::io::stdin()
        .read_line(&mut ops).unwrap();

    let n = n.trim().parse::<usize>().unwrap();
    let ops = ops.trim().chars().map(|c| {
        match c {
            'U' => -1,
            'D' => 1,
            _ => 0,
        }
    }).collect::<Vec<i8>>();

    let mut curr = 0;
    let songs = (1..).step_by(1).take(n).collect::<Vec<i32>>();
    let mut range;
    if n < 4 {
        range = &songs[..];
    } else {
        range = &songs[0..4];
    }
    for op in ops {
        match op {
            1 => {
                if curr == n - 1 {
                    curr = 0;
                    if n > 4 {
                        range = &songs[0..4];
                    }
                } else {
                    curr += 1;
                    if n > 4 && songs[curr] > range[3] {
                        range = &songs[curr-3..curr+1];
                    }
                }
            },
            -1 => {
                if curr == 0 {
                    curr = n - 1;
                    if n > 4 {
                        range = &songs[n-4..n];
                    }
                } else {
                    curr -= 1;
                    if n > 4 && songs[curr] < range[0] {
                        range = &songs[curr..curr+4];
                    }
                }
            },
            _ => {},
        }
    }

    for s in range {
        print!("{} ", s);
    }
    println!();
    println!("{}", songs[curr]);
}