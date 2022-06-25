use std::fmt::write;

fn main() {
    let mut start = (0, 0);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut moves: Vec<_> = s.trim()
            .split(';')
            .map(|s| s.parse::<Move>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
    for mv in moves {
        match mv {
            Move::W(n) => start.1 += n,
            Move::A(n) => start.0 -= n,
            Move::S(n) => start.1 -= n,
            Move::D(n) => start.0 += n,
        };
    }
    println!("{},{}", start.0, start.1);
}

enum Move {
    W(i32),
    A(i32),
    S(i32),
    D(i32),
}

impl std::str::FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 || s.len() > 3 {
            return Err(());
        }
        let num = s[1..].parse::<i32>();
        if num.is_err() { return Err(()); }
        let ret = match s.chars().next().unwrap() {
            'W' => Move::W(num.unwrap()),
            'A' => Move::A(num.unwrap()),
            'S' => Move::S(num.unwrap()),
            'D' => Move::D(num.unwrap()),
            _ => return Err(()),
        };
        Ok(ret)
    }
}
// impl std::fmt::Display for Move {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::W(n) => write!(f, "W{}", n),
//             Self::A(n) => write!(f, "A{}", n),
//             Self::S(n) => write!(f, "S{}", n),
//             Self::D(n) => write!(f, "D{}", n),
//         }
//     }
// }