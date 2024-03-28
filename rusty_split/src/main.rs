// function to split a string on a given delimiter and
// returns a two dimensional array: Vec<String>
fn  rusty_split(str: String, delimiter: char) -> Vec<String> {
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
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::rusty_split;
    #[test]
    fn test_hello_world() {
        let str = "   Hello,  \t\0world!    ";
        let del = ' ';
        let res: Vec<String> = rusty_split((&str).to_string(), del);
        assert_eq!(res[0], "Hello,");
        assert_eq!(res[1], "\t\0world!");
    }
}