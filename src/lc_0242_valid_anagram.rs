pub fn is_anagram(s: String, t: String) -> bool {
    let mut x = s.chars().collect::<Vec<char>>();
    let mut y = t.chars().collect::<Vec<char>>();

    x.sort();
    y.sort();

    x == y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()))
    }
}
