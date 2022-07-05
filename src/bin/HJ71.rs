fn main() {
    let mut reg = String::new();
    let mut exp = String::new();

    std::io::stdin()
        .read_line(&mut reg).unwrap();
    std::io::stdin()
        .read_line(&mut exp).unwrap();

    let reg = reg.trim().to_lowercase().chars().collect::<Vec<_>>();
    let exp = exp.trim().to_lowercase().chars().collect::<Vec<_>>();
    println!("{}", is_match(&reg, &exp));
}

fn is_match(s1: &[char], s2: &[char]) -> bool {
    if s1.len() == 0 && s2.len() == 0{
        return true;
    }

    if s1.len() == 0 || s2.len() == 0 {
        return false;
    }

    if s1[0] != s2[0] && s1[0] != '*' && s1[0] != '?' {
        return false;
    }
    
    if s1[0] == s2[0] || (s1[0] == '?' && s2[0].is_alphanumeric()) {
        return is_match(&s1[1..], &s2[1..]);
    }

    if s1[0] == '*' {
        return is_match(&s1[1..], s2) || is_match(s1, &s2[1..]) || is_match(&s1[1..], &s2[1..]);
    }
    
    false
}