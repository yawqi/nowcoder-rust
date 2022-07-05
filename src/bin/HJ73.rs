fn main() {
    let mut date = String::new();
    std::io::stdin()
        .read_line(&mut date);
    let date = date.trim()
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
    let year = date[0];
    let month = date[1];
    let day = date[2];

    let mut count = 0;
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 0..month {
        if i == 1 {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                count += 29;
            } else {
                count += 28;
            }
        } else {
            count += days[i];
        }
    }
    count += day;
    println!("{}", count);
}