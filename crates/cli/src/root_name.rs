use std::path::Path;

pub fn root_name_from_input_name(input: &str) -> &str {
    Path::new(input)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .trim_end_matches(".jtd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_name_from_input_name() {
        assert_eq!("foo", root_name_from_input_name("foo.jtd.json"));
        assert_eq!("foo", root_name_from_input_name("foo.jtd.yaml"));
        assert_eq!("foo", root_name_from_input_name("foo.json"));
        assert_eq!("foo", root_name_from_input_name("foo"));
        assert_eq!("", root_name_from_input_name(""));
    }
}
