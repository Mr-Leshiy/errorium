pub(crate) fn to_snake_case(s: String) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                Some('_')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c.to_ascii_lowercase()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_snake_case_test() {
        assert_eq!(to_snake_case("FooBar".to_string()), "foo_bar");
        assert_eq!(to_snake_case("fooBar".to_string()), "foo_bar");
        assert_eq!(to_snake_case("foobar".to_string()), "foobar");
        assert_eq!(to_snake_case("Foobar".to_string()), "foobar");
    }
}
