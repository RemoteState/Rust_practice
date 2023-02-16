pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => input.to_string().replace(first, &first.to_ascii_uppercase().to_string()),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut v = vec![];
    for i in words {
        let b = capitalize_first(i);
        v.push(b.to_string());
    }
    v
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut a = String::new();
    for i in words {
        if i != &" " {
            let b = capitalize_first(i);
            a += &b;
            continue;
        }
        a += i;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}