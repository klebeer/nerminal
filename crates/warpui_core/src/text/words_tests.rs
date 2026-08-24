use super::{
    DEFAULT_WORD_BOUNDARY_CHARS, is_default_word_boundary, is_subword_boundary_char,
    split_at_next_word_start,
};

#[test]
fn test_ascii_word_boundaries_unchanged() {
    for c in DEFAULT_WORD_BOUNDARY_CHARS {
        assert!(
            is_default_word_boundary(c),
            "{c:?} is in DEFAULT_WORD_BOUNDARY_CHARS and must stay a boundary"
        );
    }

    for c in [
        ',', '.', ';', ':', '!', '?', '(', ')', '[', ']', '/', '\\', '"', '\'', '-',
    ] {
        assert!(is_default_word_boundary(c), "{c:?} must be a boundary");
    }

    for c in [' ', '\t', '\n', '\r'] {
        assert!(is_default_word_boundary(c), "{c:?} must be a boundary");
    }

    for c in ['a', 'z', 'A', 'Z', '0', '9', '_'] {
        assert!(!is_default_word_boundary(c), "{c:?} must not be a boundary");
    }
}

#[test]
fn test_cjk_and_fullwidth_punctuation_are_word_boundaries() {
    for c in ['，', '。', '、', '！', '？', '：', '；'] {
        assert!(
            is_default_word_boundary(c),
            "U+{:04X} {c:?} must be a boundary",
            c as u32
        );
    }
}

#[test]
fn test_cjk_brackets_are_word_boundaries() {
    for c in ['「', '」', '『', '』', '（', '）', '【', '】', '〈', '〉'] {
        assert!(
            is_default_word_boundary(c),
            "U+{:04X} {c:?} must be a boundary",
            c as u32
        );
    }
}

#[test]
fn test_ideographic_space_is_word_boundary() {
    assert!(is_default_word_boundary('\u{3000}'));
}

#[test]
fn test_cjk_letters_and_numerals_are_not_word_boundaries() {
    // U+3005 is Lm and U+3007 is Nl: both are punctuation-looking but belong to a word.
    for c in ['々', '〇', '漢', '字', 'あ', 'ア', '一', 'Ａ', '０'] {
        assert!(
            !is_default_word_boundary(c),
            "U+{:04X} {c:?} must not be a boundary",
            c as u32
        );
    }
}

#[test]
fn test_subword_boundaries_inherit_cjk_punctuation() {
    assert!(is_subword_boundary_char('，'));
    assert!(is_subword_boundary_char('「'));
    assert!(is_subword_boundary_char('_'));
    assert!(!is_subword_boundary_char('あ'));
    assert!(!is_subword_boundary_char('a'));
}

#[test]
fn test_split_at_next_word_start_breaks_on_fullwidth_punctuation() {
    assert_eq!(split_at_next_word_start("test，字"), ("test，", "字"));
    assert_eq!(split_at_next_word_start("test,word"), ("test,", "word"));
    assert_eq!(split_at_next_word_start("漢字。かな"), ("漢字。", "かな"));
    assert_eq!(split_at_next_word_start("漢字かな"), ("漢字かな", ""));
}
