use unicode_general_category::{GeneralCategory, get_general_category};

/// The default word-boundary characters.
pub const DEFAULT_WORD_BOUNDARY_CHARS: [char; 33] = [
    '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '+', '[', '{', ']', '}',
    '\\', '|', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?', '«', '»',
];

/// Default subword-boundary characters: basically just underscores for now (snake_case)
pub const SUBWORD_BOUNDARY_CHARS: [char; 1] = ['_'];

/// Split a string slice at the next word boundary, returning before and after the word boundary
///
/// The next word boundary is the transition from not in a word (i.e. in a separator) to
/// in a word. The first slice returned goes from start of the input slice to the word boundary.
/// The second slice returns goes from the start of the new word to the end of the input slice.
pub fn split_at_next_word_start(text: &str) -> (&str, &str) {
    let mut in_word = true;
    let mut byte_index = 0;
    for c in text.chars() {
        if in_word {
            if is_default_word_boundary(c) {
                in_word = false;
            }
        } else if !is_default_word_boundary(c) {
            break;
        }
        byte_index += c.len_utf8();
    }

    text.split_at(byte_index)
}

/// Default logic for determining if a character is a word separator. Word separators are
/// whitespace or a specific set of punctuation characters.
///
/// Non-ASCII punctuation in the open/close/initial/final/other categories also separates words, so
/// CJK and fullwidth punctuation such as `，` (U+FF0C) breaks a word the same way its ASCII
/// counterpart does. The category check is restricted to non-ASCII so that ASCII selection stays
/// governed solely by [`DEFAULT_WORD_BOUNDARY_CHARS`]. Connectors (`Pc`) and dashes (`Pd`) are
/// excluded because they commonly appear inside identifiers.
pub fn is_default_word_boundary(c: char) -> bool {
    if c.is_whitespace() || DEFAULT_WORD_BOUNDARY_CHARS.contains(&c) {
        return true;
    }
    if c.is_ascii() {
        return false;
    }
    matches!(
        get_general_category(c),
        GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::OtherPunctuation
    )
}

/// Logic for determining if a character is a subword separator.
/// Subword separators include all the default word separators
/// (whitespace or a specific set of punctuation characters)
/// and subword-specific separators (underscores, for snake_case).
pub fn is_subword_boundary_char(c: char) -> bool {
    is_default_word_boundary(c) || SUBWORD_BOUNDARY_CHARS.contains(&c)
}

#[cfg(test)]
#[path = "words_tests.rs"]
mod tests;
