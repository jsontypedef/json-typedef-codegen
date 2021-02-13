pub fn comment_block(before: &str, prefix: &str, after: &str, s: &str) -> String {
    if s.is_empty() {
        return "".to_owned();
    }

    let middle = textwrap::fill(s, 80 - prefix.len())
        .lines()
        .map(|s| format!("{}{}", prefix, s))
        .collect::<Vec<_>>()
        .join("\n");

    if before.is_empty() && after.is_empty() {
        format!("{}\n", middle)
    } else {
        format!("{}\n{}\n{}\n", before, middle, after)
    }
}
