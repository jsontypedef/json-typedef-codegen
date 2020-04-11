use handlebars::handlebars_helper;
use textwrap;

// A Handlebars helper function that wraps comments.
//
// To enable JavaDoc-style comments, with a leading "/**" and tailing "*/", use
// `first_line` and `last_line`. Leave `first_line` empty to only use `prefix`.
handlebars_helper!(comment: |max_width: u64, first_line: str, prefix: str, last_line: str, s: str| {
    if s.is_empty() {
        "".to_owned()
    } else {
        let lines: Vec<_> = textwrap::fill(s, max_width as usize - prefix.len()).lines().map(|line| format!("{}{}", prefix, line)).collect();

        if first_line.is_empty() {
            lines.join("\n")
        } else {
            format!("{}\n{}\n{}", first_line, lines.join("\n"), last_line)
        }
    }
});
