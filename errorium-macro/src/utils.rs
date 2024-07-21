#[allow(dead_code)]
pub(crate) fn to_snake_case(s: &str) -> String {
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
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
        assert_eq!(to_snake_case("fooBar"), "foo_bar");
        assert_eq!(to_snake_case("foobar"), "foobar");
        assert_eq!(to_snake_case("Foobar"), "foobar");
    }
}
