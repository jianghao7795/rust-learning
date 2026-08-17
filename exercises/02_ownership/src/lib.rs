pub fn first_word(text: &str) -> &str {
    match text.find(char::is_whitespace) {
        Some(index) => &text[..index],
        None => text,
    }
}

pub fn append_world(text: &mut String) {
    if !text.is_empty() {
        text.push(' ');
    }
    text.push_str("world");
}

pub fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub fn largest(values: &[i32]) -> Option<&i32> {
    values.iter().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_first_word() {
        assert_eq!(first_word("hello rust"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn modifies_borrowed_string() {
        let mut text = String::from("hello");
        append_world(&mut text);
        assert_eq!(text, "hello world");
    }

    #[test]
    fn returns_a_borrowed_result() {
        let left = String::from("short");
        let right = String::from("a longer string");
        assert_eq!(longer(&left, &right), right);
    }

    #[test]
    fn finds_largest_value() {
        assert_eq!(largest(&[3, 9, 4]), Some(&9));
        assert_eq!(largest(&[]), None);
    }
}
