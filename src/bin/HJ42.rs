const digits: [&str ;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const tens: [&str; 10] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const tenss: [&str; 8] = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    let mut v = vec![];
    if let Some(bills) = parse_billions(n) {
        v.push(bills);
    }
    if let Some(mills) = parse_millions(n) {
        v.push(mills);
    }
    if let Some(thous) = parse_thousands(n) {
        v.push(thous);
    }
    if let Some(hunds) = parse_hunderds(n) {
        v.push(hunds);
    }
    println!("{}", v.join(" "));
}
fn parse_hunderds(n: u64) -> Option<String> {
    let mut num = n % 1000;
    if num == 0 {
        return None;
    }
    let mut ret = vec![];
    if num >= 100 {
        let hundred = (num / 100) as usize;
        ret.push(digits[hundred]);
        ret.push("hundred");
        num %= 100;
        if num != 0 {
            ret.push("and")
        }
    }

    if num >= 10 {
        if num < 20 {
            ret.push(tens[num as usize - 10]);
            num = 0;
        } else {
            ret.push(tenss[(num as usize - 20)/10]);
        }
        num %= 10;
    }

    if num != 0 {
        ret.push(digits[num as usize]);
    }
    Some(ret.join(" "))
}

fn parse_thousands(n: u64) -> Option<String> {
    match parse_hunderds((n % 1000000) / 1000) {
        Some(mut hds) => {
            hds.push_str(" thousand");
            Some(hds)
        },
        None => None,
    }
}

fn parse_millions(n: u64) -> Option<String> {
    match parse_hunderds((n % 1000000000) / 1000000) {
        Some(mut hds) => {
            hds.push_str(" million");
            Some(hds)
        },
        None => None,
    }
}

fn parse_billions(n: u64) -> Option<String> {
    match parse_hunderds(n / 1000000000) {
        Some(mut hds) => {
            hds.push_str(" billion");
            Some(hds)
        },
        None => None,
    }
}