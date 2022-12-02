fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_word(){
         let mut s = String::from("hello world");

        let word = first_word(&s);

        // s.clear(); // error!

        assert_eq!(word,&s[..5])

    }
}