use std::{collections::{HashMap, HashSet}, process::Command};
fn main() {
    let mut map = HashMap::new();
    let commands = vec![vec!["reset"], vec!["reset", "board"], vec!["board", "add"], vec!["board", "delete"], vec!["reboot", "backplane"], vec!["backplane", "abort"]];
    let results = vec!["reset what", "board fault", "where to add", "no board at all", "impossible", "install first"];
    for i in 0..commands.len() {
        map.insert(commands[i].clone(), results[i]);
    }

    'outer: loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }
        let v = s.trim().split(" ").collect::<Vec<_>>();
        let cmds = {
            let mut choose = vec![];
            for command in &commands {
                if command[0].starts_with(&v[0]) && command.len() == v.len() {
                    choose.push(command.clone());
                }
            }
            choose
        };

        if cmds.is_empty() || (cmds.len() > 1 && v.len() == 1) {
            println!("unknown command");
            continue 'outer;
        }

        let cmd = if v.len() > 1 {
            let mut choose = vec![];
            for cmd in cmds {
                if cmd[1].starts_with(v[1]) {
                    if choose.is_empty() {
                        choose = cmd.clone();
                    } else {
                        println!("unknown command");
                        continue 'outer;
                    }
                }
            }
            choose
        } else {
            cmds[0].clone()
        };

        match map.get(&cmd) {
            Some(ret) => println!("{}", ret),
            None => println!("unknown command"),
        }
    }
}