

// slow but correct implementation
pub fn search(query: &String, content: String) -> Vec<String> {

    let mut res: Vec<String> = vec![];

    for i in content.lines() { // so that 's' is not consumed
        if i.contains(query) {
            res.push(i.to_string());
        }
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search() {
        assert_eq!(vec!["line2".to_string()], search(&"ne2".to_string(), "
line1
line2
        ".to_string()));
    }

    #[test]
    #[should_panic]
    fn null_search() {
        assert_ne!(vec![] as Vec<String>, search(&"nada".to_string(), "
line1
line2
    ".to_string()));
    }

    #[test]
    fn multi_search() {
        assert_eq!(vec!["bar".to_string(), "baz".to_string()], 
            search(&"ba".to_string(), "
bar
foo
baz
    ".to_string()));
    }

}