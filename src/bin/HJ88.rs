use std::str::FromStr;

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let result = s.trim().split("-").collect::<Vec<_>>();
    let inputs = result.iter().map(|n| n.parse::<Pokers>().unwrap()).collect::<Vec<_>>();
    match cmp_cards(&inputs[0], &inputs[1]) {
        1 => println!("{}", result[0]),
        -1 => println!("{}", result[1]),
        _ => println!("ERROR"),
    }
}

enum Pokers {
    Single(i32),
    Double(i32),
    Five(i32),
    Triple(i32),
    Bomb(i32),
    Jokers,
}

impl Pokers {
    fn priority(&self) -> i32 {
        match self {
            Self::Single(_) => 1,
            Self::Double(_) => 2,
            Self::Triple(_) => 3,
            Self::Five(_) => 4,
            Self::Bomb(_) => 5,
            Self::Jokers => 6,
        }
    }

    fn get_card(&self) -> i32 {
        match self {
            Self::Single(i) => *i,
            Self::Double(i) => *i,
            Self::Triple(i) => *i,
            Self::Five(i) => *i,
            Self::Bomb(i) => *i,
            Self::Jokers => 0,
        }
    }
}

impl FromStr for Pokers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.split(" ").collect::<Vec<_>>();
        let num1 = match cards[0] {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            "joker" => 100,
            "JOKER" => 200,
            "2" => 50,
            _ => {
                cards[0].parse::<i32>().unwrap()
            }
        };
        match cards.len() {
            1 => {
                return Ok(Self::Single(num1));
            },
            2 => {
                let num2 = match cards[1] {
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "joker" => 100,
                    "JOKER" => 200,
                    "2" => 50,
                    _ => {
                        cards[0].parse::<i32>().unwrap()
                    }
                };

                if num1 != num2 {
                    return Ok(Self::Jokers);
                } else {
                    return Ok(Self::Double(num1));
                }
            },
            3 => {
                return Ok(Self::Triple(num1));
            },
            4 => {
                return Ok(Self::Bomb(num1));
            },
            5 => {
                return Ok(Self::Five(num1));
            },
            _ => return Err(()),
        }
    }
}

fn cmp_cards(p1: &Pokers, p2: &Pokers) -> i32 {
    let pri1 = p1.priority();
    let pri2 = p2.priority();

    if pri1 != pri2 {
        if pri1 >= 5 || pri2 >= 5 {
            if pri1 > pri2 {
                return 1;
            } else if pri1 < pri2 {
                return -1;
            }
        }
        return 0;
    } else {
        let n1 = p1.get_card();
        let n2 = p2.get_card();
        if n1 < n2 {
            return -1;
        } else {
            return 1;
        }
    }
}