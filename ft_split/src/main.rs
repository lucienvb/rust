// function to split a string on a given delimiter and
// returns a two dimensional array: Vec<String>
fn  split(str: String, delimiter: char) -> Vec<String> {
    let new_str = str.trim();
    let len = new_str.len() - 1;
    let mut res: Vec<String> = Vec::new();
    let mut current_string: String = String::new();

    for (i, c) in new_str.chars().enumerate() {
        if c != delimiter {
            current_string.push(c);
        }
        if c == delimiter || i == len {
            if !current_string.is_empty() {
                res.push(current_string.clone());
                current_string.clear();
            }
        }
        println!("{}", c);
    }
    res
}

fn  main() {
    let str = "   Hello,  \0world!    ";
    let del = ' ';
    let res: Vec<String> = split((&str).to_string(), del);
    println!("res[0]: {}", res[0]);
    println!("res[1]: {}", res[1]);
}
