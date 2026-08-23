use super::*;

#[test]
fn the_shared_margin_is_removed() {
    let copied = "  first line\n  second line\n";

    assert_eq!(strip_common_indent(copied), "first line\nsecond line\n");
}

#[test]
fn relative_indentation_survives() {
    // The margin is the program's; everything past it belongs to the code.
    let copied = "  def resolve(rows):\n      out = []\n      return out\n";

    assert_eq!(
        strip_common_indent(copied),
        "def resolve(rows):\n    out = []\n    return out\n"
    );
}

#[test]
fn a_blank_line_does_not_collapse_the_margin() {
    let copied = "  first paragraph\n\n  second paragraph\n";

    assert_eq!(
        strip_common_indent(copied),
        "first paragraph\n\nsecond paragraph\n"
    );
}

#[test]
fn a_line_of_only_spaces_counts_as_blank_and_comes_back_empty() {
    let copied = "  first\n   \n  second";

    assert_eq!(strip_common_indent(copied), "first\n\nsecond");
}

#[test]
fn one_unindented_line_leaves_everything_alone() {
    // Nothing is shared, so there is no margin to remove.
    let copied = "  indented\nflush left\n  indented again\n";

    assert_eq!(strip_common_indent(copied), copied);
}

#[test]
fn a_selection_starting_mid_line_is_a_no_op() {
    // The first line arrives with its margin already cut off by the selection,
    // which leaves nothing shared.
    let copied = "line/deployments/orders-api\n  view=timeline&expand=locks\n";

    assert_eq!(strip_common_indent(copied), copied);
}

#[test]
fn tabs_are_left_alone() {
    // How wide a tab is depends on someone else's settings, so a margin that
    // mixes them cannot be measured without guessing.
    let copied = "  \tfirst\n  \tsecond\n";

    assert_eq!(strip_common_indent(copied), copied);
}

#[test]
fn text_with_no_margin_is_returned_unchanged() {
    let copied =
        "09:41:02 INFO  orders-api    acquired lock\n09:41:07 INFO  orders-api    lag 1420ms\n";

    assert_eq!(strip_common_indent(copied), copied);
}

#[test]
fn a_single_line_loses_its_margin() {
    assert_eq!(strip_common_indent("    just one"), "just one");
}

#[test]
fn empty_input_is_unchanged() {
    assert_eq!(strip_common_indent(""), "");
    assert_eq!(strip_common_indent("\n\n"), "\n\n");
}

#[test]
fn trailing_newline_structure_is_preserved() {
    // Losing or gaining a trailing newline changes what a paste looks like.
    assert_eq!(strip_common_indent("  a\n  b"), "a\nb");
    assert_eq!(strip_common_indent("  a\n  b\n"), "a\nb\n");
}

#[test]
fn a_deeper_line_keeps_the_difference() {
    let copied = "    outer\n        inner\n    outer again\n";

    assert_eq!(
        strip_common_indent(copied),
        "outer\n    inner\nouter again\n"
    );
}

#[test]
fn padding_at_the_end_of_a_line_is_removed() {
    let copied = "09:41:02 INFO  acquired lock          \n09:41:07 INFO  lag 1420ms       \n";

    assert_eq!(
        trim_trailing_spaces(copied),
        "09:41:02 INFO  acquired lock\n09:41:07 INFO  lag 1420ms\n"
    );
}

#[test]
fn a_line_of_padding_comes_back_empty() {
    // A row a program cleared reads as blank but arrives as a run of spaces as
    // wide as the window.
    let copied = "first\n                                        \nsecond";

    assert_eq!(trim_trailing_spaces(copied), "first\n\nsecond");
}

#[test]
fn leading_whitespace_is_not_touched_by_the_trim() {
    let copied = "    indented line    \n";

    assert_eq!(trim_trailing_spaces(copied), "    indented line\n");
}

#[test]
fn text_without_padding_is_returned_unchanged() {
    let copied = "no padding here\nnor here\n";

    assert_eq!(trim_trailing_spaces(copied), copied);
}

#[test]
fn the_trim_and_the_dedent_compose() {
    // Together they are what turns a copied message into something that can be
    // pasted somewhere else.
    let copied = "  first line      \n                  \n  second line   \n";

    assert_eq!(
        strip_common_indent(&trim_trailing_spaces(copied)),
        "first line\n\nsecond line\n"
    );
}
