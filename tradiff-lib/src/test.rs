
use pretty_assertions::assert_eq;

use crate::{dquote, ftildes, percent, tilde, ExplicitTraEntry, TraComment, TraEntry, TraFileParser, TraFragment, WeiduStringLit};
use crate::TraEntryContent::Explicit;

#[test]
fn multiple_string_variants() {
    let input = r#"
    @1 = ~aaa~
    @2 = "bbb"
    @3 = %ccc%
    @4 = ~~~~~abc~~abc~~~~~
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::simplest(tilde!("aaa")))}),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::simplest(dquote!("bbb")))}),
            TraFragment::Entry(TraEntry { id: 3, content: Explicit(ExplicitTraEntry::simplest(percent!("ccc")))}),
            TraFragment::Entry(TraEntry { id: 4, content: Explicit(ExplicitTraEntry::simplest(ftildes!("abc~~abc")))}),
        ])
    )
}

#[test]
fn multiple_entries_on_single_line() {
    let input = r#"
    @1 = ~aaa~ @2 = "bbb"  @3 = %ccc% @4 = ~~~~~abc~~abc~~~~~
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::simplest(tilde!("aaa")))}),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::simplest(dquote!("bbb")))}),
            TraFragment::Entry(TraEntry { id: 3, content: Explicit(ExplicitTraEntry::simplest(percent!("ccc")))}),
            TraFragment::Entry(TraEntry { id: 4, content: Explicit(ExplicitTraEntry::simplest(ftildes!("abc~~abc")))}),
        ])
    )
}

#[test]
fn with_enclosed_comments_between_entries() {
    let input = r#"
    /* comment 1 */
    @1 = ~aaa~
    /* comment 2 */
    @2 = "bbb"
    /* comment 3 */
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Comment(TraComment::Enclosed(" comment 1 ".to_string())),
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::simplest(tilde!("aaa")))}),
            TraFragment::Comment(TraComment::Enclosed(" comment 2 ".to_string())),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::simplest(dquote!("bbb")))}),
            TraFragment::Comment(TraComment::Enclosed(" comment 3 ".to_string())),
        ])
    )
}

#[test]
fn with_end_of_line_comments() {
    let input = r#"
    // comment 1
    @1 = ~aaa~ // comment 2
    @2 = "bbb"// comment 3
    // comment 4
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Comment(TraComment::EndOfLine(" comment 1".to_string())),
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::simplest(tilde!("aaa")))}),
            TraFragment::Comment(TraComment::EndOfLine(" comment 2".to_string())),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::simplest(dquote!("bbb")))}),
            TraFragment::Comment(TraComment::EndOfLine(" comment 3".to_string())),
            TraFragment::Comment(TraComment::EndOfLine(" comment 4".to_string())),
        ])
    )
}

#[test]
fn with_multiline_enclosed_comment() {
    let input = r#"
    @1 = ~aaa~
    /*
    comment 2
    */
    @2 = "bbb"
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::simplest(tilde!("aaa")))}),
            TraFragment::Comment(TraComment::Enclosed("\n    comment 2\n    ".to_string())),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::simplest(dquote!("bbb")))}),
        ])
    )
}

#[test]
fn with_en_of_line_comment_and_junk_on_the_next_line() {
    let input = r#"
    @1 = ~aaa~
    // comment 2
    junk
    @2 = "bbb"
    "#;

    let mut errors = Vec::new();

    assert!(TraFileParser::new().parse(&mut errors, &input).is_err())
}


#[test]
fn with_female_variant() {
    let input = r#"
    @1 = ~aaa~ ~aab~
    @2 = "bbb" "bbc"
    @3 = %ccc% %ccd%
    @4 = ~~~~~abc~~abc~~~~~ ~~~~~bca~~bca~~~~~
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::with_female(tilde!("aaa"), tilde!("aab")))}),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::with_female(dquote!("bbb"), dquote!("bbc")))}),
            TraFragment::Entry(TraEntry { id: 3, content: Explicit(ExplicitTraEntry::with_female(percent!("ccc"), percent!("ccd")))}),
            TraFragment::Entry(TraEntry { id: 4, content: Explicit(ExplicitTraEntry::with_female(ftildes!("abc~~abc"), ftildes!("bca~~bca")))}),
        ])
    )
}

#[test]
fn with_male_sound() {
    let input = r#"
    @1 = ~aaa~ [ASOUND]
    @2 = "bbb" [BSOUND]
    @3 = %ccc% [CSOUND]
    @4 = ~~~~~abc~~abc~~~~~[DSOUND]
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry { id: 1, content: Explicit(ExplicitTraEntry::with_sound(tilde!("aaa"), "ASOUND"))}),
            TraFragment::Entry(TraEntry { id: 2, content: Explicit(ExplicitTraEntry::with_sound(dquote!("bbb"), "BSOUND"))}),
            TraFragment::Entry(TraEntry { id: 3, content: Explicit(ExplicitTraEntry::with_sound(percent!("ccc"), "CSOUND"))}),
            TraFragment::Entry(TraEntry { id: 4, content: Explicit(ExplicitTraEntry::with_sound(ftildes!("abc~~abc"), "DSOUND"))}),
        ])
    )
}

#[test]
fn with_male_sound_and_female_variant() {
    let input = r#"
    @1 = ~aaa~ [ASOUND] ~aab~
    @2 = "bbb" [BSOUND] "bbc"
    @3 = %ccc% [CSOUND] %ccd%
    @4 = ~~~~~abc~~abc~~~~~[DSOUND] ~~~~~bca~~bca~~~~~
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry {
                id: 1,
                content: Explicit(ExplicitTraEntry::new(tilde!("aaa"), Some("ASOUND"), Some(tilde!("aab")), None))
            }),
            TraFragment::Entry(TraEntry {
                id: 2,
                content: Explicit(ExplicitTraEntry::new(dquote!("bbb"), Some("BSOUND"), Some(dquote!("bbc")), None))
            }),
            TraFragment::Entry(TraEntry {
                id: 3,
                content: Explicit(ExplicitTraEntry::new(percent!("ccc"), Some("CSOUND"), Some(percent!("ccd")), None))
            }),
            TraFragment::Entry(TraEntry {
                id: 4,
                content: Explicit(ExplicitTraEntry::new(ftildes!("abc~~abc"), Some("DSOUND"), Some(ftildes!("bca~~bca")), None))
            }),
        ])
    )
}

#[test]
fn with_all_elements_in_tra_entry() {
    let input = r#"
    @1 = ~aaa~ [ASOUND] ~aab~ [FASOUND]
    @2 = "bbb" [BSOUND] "bbc" [FBSOUND]
    @3 = %ccc% [CSOUND] %ccd% [FCSOUND]
    @4 = ~~~~~abc~~abc~~~~~[DSOUND] ~~~~~bca~~bca~~~~~ [FDSOUND]
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry {
                id: 1,
                content: Explicit(ExplicitTraEntry::new(tilde!("aaa"), Some("ASOUND"), Some(tilde!("aab")), Some("FASOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 2,
                content: Explicit(ExplicitTraEntry::new(dquote!("bbb"), Some("BSOUND"), Some(dquote!("bbc")), Some("FBSOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 3,
                content: Explicit(ExplicitTraEntry::new(percent!("ccc"), Some("CSOUND"), Some(percent!("ccd")), Some("FCSOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 4,
                content: Explicit(ExplicitTraEntry::new(ftildes!("abc~~abc"), Some("DSOUND"), Some(ftildes!("bca~~bca")), Some("FDSOUND")))
            }),
        ])
    )
}

#[test]
fn with_all_elements_in_tra_entry_and_eol_comment() {
    let input = r#"
    @1 = ~aaa~ [ASOUND] ~aab~ [FASOUND] // comment 1
    @2 = "bbb" [BSOUND] "bbc" [FBSOUND] // comment 2
    @3 = %ccc% [CSOUND] %ccd% [FCSOUND] // comment 3
    @4 = ~~~~~abc~~abc~~~~~[DSOUND] ~~~~~bca~~bca~~~~~ [FDSOUND] // comment 4
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry {
                id: 1,
                content: Explicit(ExplicitTraEntry::new(tilde!("aaa"), Some("ASOUND"),
                                                        Some(tilde!("aab")), Some("FASOUND")))
            }),
            TraFragment::Comment(TraComment::EndOfLine(" comment 1".to_string())),
            TraFragment::Entry(TraEntry {
                id: 2,
                content: Explicit(ExplicitTraEntry::new(dquote!("bbb"), Some("BSOUND"), Some(dquote!("bbc")), Some("FBSOUND")))
            }),
            TraFragment::Comment(TraComment::EndOfLine(" comment 2".to_string())),
            TraFragment::Entry(TraEntry {
                id: 3,
                content: Explicit(ExplicitTraEntry::new(percent!("ccc"), Some("CSOUND"), Some(percent!("ccd")), Some("FCSOUND")))
            }),
            TraFragment::Comment(TraComment::EndOfLine(" comment 3".to_string())),
            TraFragment::Entry(TraEntry {
                id: 4,
                content: Explicit(ExplicitTraEntry::new(ftildes!("abc~~abc"), Some("DSOUND"), Some(ftildes!("bca~~bca")), Some("FDSOUND")))
            }),
            TraFragment::Comment(TraComment::EndOfLine(" comment 4".to_string())),
        ])
    )
}

/* this is not implemented
#[test]
fn with_all_elements_in_tra_entry_and_middle_of_line_comment() {
    let input = r#"
    @1 = /* comment 1 */~aaa~ [ASOUND] ~aab~ [FASOUND]
    @2 = "bbb" /* comment 2 */[BSOUND] "bbc" [FBSOUND]
    @3 = %ccc% [CSOUND] /* comment 3 */%ccd% [FCSOUND]
    @4 = ~~~~~abc~~abc~~~~~[DSOUND] ~~~~~bca~~bca~~~~~ /* comment 4 */[FDSOUND]
    "#;

    let mut errors = Vec::new();

    assert_eq!(
        TraFileParser::new().parse(&mut errors, &input),
        Ok(vec![
            TraFragment::Entry(TraEntry {
                id: 1,
                content: Explicit(ExplicitTraEntry::new("aaa", Some("ASOUND"), Some("aab"), Some("FASOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 2,
                content: Explicit(ExplicitTraEntry::new("bbb", Some("BSOUND"), Some("bbc"), Some("FBSOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 3,
                content: Explicit(ExplicitTraEntry::new("ccc", Some("CSOUND"), Some("ccd"), Some("FCSOUND")))
            }),
            TraFragment::Entry(TraEntry {
                id: 4,
                content: Explicit(ExplicitTraEntry::new("abc~~abc", Some("DSOUND"), Some("bca~~bca"), Some("FDSOUND")))
            }),
        ])
    )
}
*/
