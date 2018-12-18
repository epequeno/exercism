use unicode_segmentation::UnicodeSegmentation;

// https://stackoverflow.com/a/27996791
pub fn reverse(input: &str) -> String {
    input
        .graphemes(true)
        .rev()
        .flat_map(|g| g.chars())
        .collect::<String>()
}
