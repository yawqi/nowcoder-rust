use std::io::Read;

fn main() {
    let mut n = String::new();
    let mut order = String::new();
    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut order);
    let n = n.trim().parse::<usize>().unwrap();
    let order = order.trim().parse::<u8>().unwrap();
    let mut vs = vec![];
    for i in 0..n {
        let mut student = String::new();
        std::io::stdin()
            .read_line(&mut student);
        let v = student.trim().split(" ").collect::<Vec<_>>();
        let s = Student {
            name: v[0].to_string(),
            grade: v[1].parse().unwrap(),
            occur: i as u8,
        };
        vs.push(s);
    }
    vs.sort_by(|s1, s2| {
        if s1.grade != s2.grade {
            if order == 0 {
                s1.grade.cmp(&s2.grade)
            } else {
                s2.grade.cmp(&s1.grade)
            }
        } else {
            s1.occur.cmp(&s2.occur)
        }
    });

    for s in vs {
        println!("{} {}", s.name, s.grade);
    }
}

struct Student {
    name: String,
    grade: u8,
    occur: u8,
}