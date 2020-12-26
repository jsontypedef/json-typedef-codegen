pub fn comment_block(before: &str, prefix: &str, after: &str, s: &str) -> String {
    let middle = textwrap::fill(s, 80 - prefix.len())
        .lines()
        .map(|s| format!("{}{}", prefix, s))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}\n{}\n", before, middle, after)
}
