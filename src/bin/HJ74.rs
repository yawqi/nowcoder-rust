fn main() {
    let mut cmd = String::new();
    std::io::stdin()
        .read_line(&mut cmd);
    
    let args = cmd.trim().split(" ").collect::<Vec<_>>();
    let mut string_cmd = vec![];
    let mut is_string = false;
    let mut results = vec![];
    for arg in args {
        if is_string {
            if arg.ends_with("\"") {
                string_cmd.push(arg[0..arg.len()-1].to_string());
                results.push(string_cmd.join(" "));
                is_string = false;
            } else {
                string_cmd.push(arg.to_string());
            }
        } else {
            if arg.starts_with("\"") {
                if !arg.ends_with("\"") {
                    is_string = true;
                    string_cmd.push(arg[1..].to_string());
                } else {
                    results.push(arg[1..arg.len()-1].to_string()); 
                }
            } else {
                results.push(arg.to_string());
            }
        }
    }

    println!("{}", results.len());
    for ret in results {
        println!("{}", ret);
    }
}