fn main() {

    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let mut score = 0u32;
    let s = s.trim().to_string();
    if s.len() <= 4 {
        score += 5;
    } else if s.len() <= 7 {
        score += 10;
    } else {
        score += 25
    }

    let mut letter_count = 0;
    let mut digit_count = 0;
    let mut other_count = 0;
    s.chars()
        .for_each(|c| {
            match c {
                c if c.is_alphabetic() => {
                    if c.is_uppercase() {
                        letter_count |= 2;
                    }
                    if c.is_lowercase() {
                        letter_count |= 1;
                    }
                },
                c if c.is_digit(10) => digit_count += 1,
                _ => other_count += 1,
            }
        });
    if letter_count > 0 {
        if letter_count == 3 {
            score += 20;
        } else {
            score += 10;
        }
    }

    if digit_count > 0 {
        if digit_count > 1 {
            score += 20;
        } else {
            score += 10;
        }
    }

    if other_count > 0 {
        if other_count > 1 {
            score += 25;
        } else {
            score += 10;
        }
    }

    if digit_count > 0 && letter_count > 0 {
        if other_count > 0 {
            if letter_count == 3 {
                score += 5;
            } else {
                score += 3;
            }
        } else {
            score += 2;
        }
    }
    let ret = match score {
        90..=100 => "VERY_SECURE",
        80..=89 => "SECURE",
        70..=79 => "VERY_STRONG",
        60..=69 => "STRONG",
        50..=59 => "AVERAGE",
        25..=49 => "WEAK",
        0..=24 => "VERY_WEAK",
        _ => ""
    };
    println!("{}", ret);
}