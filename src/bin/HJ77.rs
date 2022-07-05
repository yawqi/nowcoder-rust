use std::collections::VecDeque;

fn main() {
    let mut n = String::new();
    let mut trains = String::new();

    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut trains);
    let n = n.trim().parse::<usize>().unwrap();
    let mut trains = trains.trim().split(" ").map(|n| n.parse::<u32>().unwrap()).collect::<VecDeque<_>>();
    let mut results = vec![];
    let mut curr_in = vec![];
    let mut curr_out = vec![];
    out_station(&mut trains, &mut curr_in, &mut curr_out, &mut results);
    results.sort();
    for ret in results {
        println!("{}", ret.join(" "));
    }
}

fn out_station(not_in: &mut VecDeque<u32>, curr_in: &mut Vec<u32>, curr_out: &mut Vec<String>, results: &mut Vec<Vec<String>>) {
    if not_in.is_empty() && curr_in.is_empty() {
        results.push(curr_out.clone());
        return;
    }

    if !not_in.is_empty() {
        let in_train = not_in.pop_front().unwrap();
        curr_in.push(in_train);
        out_station(not_in, curr_in, curr_out, results);
        curr_in.pop();
        not_in.push_front(in_train);
    }

    if !curr_in.is_empty() {
        let out_train = curr_in.pop().unwrap();
        curr_out.push(out_train.to_string());
        out_station(not_in, curr_in, curr_out, results);
        curr_out.pop();
        curr_in.push(out_train);
    }
}