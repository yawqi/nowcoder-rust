use std::vec;

fn main() {
    let mut mn = String::new();
    std::io::stdin()
        .read_line(&mut mn).unwrap();
    let mn: Vec<_> = mn.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect();
    let mut maze = vec![];
    for _ in 0..mn[0] {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line).unwrap();
        maze.push(
                line
                .trim()
                .split(" ")
                .map(|s| {
                let n = s.parse::<i32>().unwrap();
                if n == 1 {
                    true
                } else {
                    false
                }
                })
                .collect::<Vec<_>>()
            );
    }
    let mut gone = maze.clone();
    let mut path = vec![(0,0)];
    let mut paths = None;
    move_to(0, 0, &maze, &mut gone, path, &mut paths);
    if let Some(p) = paths {
        for mov in p {
            println!("({},{})", mov.0, mov.1);
        }
    }
}

fn move_to(r: usize, c: usize, maze: &Vec<Vec<bool>>, gone: &mut Vec<Vec<bool>>, path: Vec<(usize, usize)>, paths: &mut Option<Vec<(usize, usize)>>) {
    let mut path = path;
    gone[r][c] = true;
    if r == maze.len()-1 && c == maze[0].len()-1 {
        if paths.is_none() {
            *paths = Some(path);
            return;
        }
        let p = paths.take().unwrap();
        if p.len() > path.len() {
            *paths = Some(path);
        } else {
            *paths = Some(p);
        }
        return;
    }

    if r < maze.len() - 1 && !maze[r+1][c] && !gone[r+1][c] {
        path.push((r+1, c));
        move_to(r+1, c, maze, gone, path.clone(), paths);
        path.pop();
    }

    if r > 0 && !maze[r-1][c] && !gone[r-1][c] {
        path.push((r-1, c));
        move_to(r-1, c, maze, gone, path.clone(), paths);
        path.pop();
    }

    if c < maze[0].len() - 1 && !maze[r][c+1] && !gone[r][c+1] {
        path.push((r, c+1));
        move_to(r, c+1, maze, gone, path.clone(), paths);
        path.pop();
    }

    if c > 0 && !maze[r][c-1] && !gone[r][c-1] {
        path.push((r, c-1));
        move_to(r, c-1, maze, gone, path.clone(), paths);
        path.pop();
    }
}