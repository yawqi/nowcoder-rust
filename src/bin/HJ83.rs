fn main() {
    let mut results = vec![];
    loop {
        let mut mn = String::new();
        let mut swap = String::new();
        let mut row = String::new();
        let mut col = String::new();
        let mut pos = String::new();

        let bytes = std::io::stdin().read_line(&mut mn).unwrap();
        if bytes == 0 {
            break;
        }
        std::io::stdin().read_line(&mut swap).unwrap();
        std::io::stdin().read_line(&mut row).unwrap();
        std::io::stdin().read_line(&mut col).unwrap();
        std::io::stdin().read_line(&mut pos).unwrap();

        let mut mn = mn.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let swap = swap.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let row = row.trim().parse::<usize>().unwrap();
        let col = col.trim().parse::<usize>().unwrap();
        let pos = pos.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut result = vec![];
        if mn[0] > 9 || mn[1] > 9 {
            result.push(-1);
            if mn[0] > 9 {
                mn[0] = 9;
            }
            if mn[1] > 9 {
                mn[1] = 9;
            }
        } else {
            result.push(0);
        }

        if swap[0] < mn[0] && swap[1] < mn[1] && swap[2] < mn[0] && swap[3] < mn[1] {
            result.push(0);
        } else {
            result.push(-1);
        }

        if row >= mn[0] || mn[0] == 9 {
            result.push(-1);
        } else {
            result.push(0);
        }

        if col >= mn[1] || mn[1] == 9 {
            result.push(-1);
        } else {
            result.push(0);
        }
        if pos[0] >= mn[0] || pos[1] >= mn[1] {
            result.push(-1);
        } else {
            result.push(0);
        }
        results.push(result);
    }

    for r in results {
        for n in r {
            println!("{}", n);
        }
    }
}