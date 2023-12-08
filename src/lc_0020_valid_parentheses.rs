pub fn is_valid(s: String) -> bool {
    let mut stack = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if let Some(pair) = stack.pop() {
                    if pair != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_valid("()".to_string()));
    }
    #[test]
    fn ex2() {
        assert!(is_valid("(){}[]".to_string()))
    }
}
