pub fn is_palindrome(s: String) -> bool {
    let forward: String = s
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect();

    let backward: String = forward.chars().rev().collect();

    forward == backward
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }
}
