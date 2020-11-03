pub fn simple_comment_block(max_width: usize, prefix: &str, s: &str) -> String {
    textwrap::fill(s, max_width - prefix.len())
        .lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn surrounded_comment_block(
    max_width: usize,
    before: &str,
    prefix: &str,
    after: &str,
    s: &str,
) -> String {
    if s.is_empty() {
        return "".to_owned();
    }

    format!(
        "{}\n{}\n{}\n",
        before,
        simple_comment_block(max_width, prefix, s),
        after
    )
}
