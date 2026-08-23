//! Cleaning up terminal text on its way to the clipboard.

/// Removes the spaces a program left at the end of each line.
///
/// A cell holds `\0` until something is written to it, and a program that
/// redraws its own output clears the rest of a row by writing spaces over it.
/// Those spaces are indistinguishable from typed ones once they are in the
/// grid, so they are copied out with the text: a line comes back padded to the
/// width of the window, and a line that looks blank comes back as a hundred
/// spaces.
///
/// Trailing whitespace on a terminal row is layout, never content, so it goes.
/// This matches what iTerm2 does on copy.
pub fn trim_trailing_spaces(text: &str) -> String {
    text.split('\n')
        .map(|line| line.trim_end_matches([' ', '\t']))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Removes the left margin that every line of `text` shares.
///
/// A program that lays out its own output indents every line it prints, and
/// that margin is the program's, not the text's. Pasting it into a chat message
/// or an editor carries the margin along, which is why people run the text
/// through an editor first.
///
/// Only a prefix present on *every* non-blank line is removed, so nothing that
/// distinguishes one line from another is touched: relative indentation inside
/// code survives untouched. A selection that starts mid-line has a first line
/// with no margin at all, which makes the shared prefix empty and the whole
/// thing a no-op.
///
/// Lines starting with a tab are left alone entirely. Mixing tabs and spaces
/// makes "how wide is this margin" a question about someone else's tab stops,
/// and guessing wrong silently corrupts the text.
pub fn strip_common_indent(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();

    let mut common_indent = usize::MAX;
    for line in &lines {
        if line.trim().is_empty() {
            // A blank line has no margin to speak of and must not drag the
            // shared prefix down to zero.
            continue;
        }
        let indent = line.len() - line.trim_start_matches(' ').len();
        if line[indent..].starts_with('\t') {
            return text.to_owned();
        }
        common_indent = common_indent.min(indent);
        if common_indent == 0 {
            return text.to_owned();
        }
    }

    if common_indent == 0 || common_indent == usize::MAX {
        return text.to_owned();
    }

    lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else {
                &line[common_indent..]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "copy_text_tests.rs"]
mod tests;
