use super::check_lexing;
use expect_test::expect;

#[test]
fn line_comment() {
    check_lexing(
        r"
// line
//// line as well
/// outer doc line
//! inner doc line
",
        expect![[r#"
            Token { kind: Whitespace, text: "\n", range: 0..1 }
            Token { kind: LineComment(None), text: "// line", range: 1..8 }
            Token { kind: Whitespace, text: "\n", range: 8..9 }
            Token { kind: LineComment(None), text: "//// line as well", range: 9..26 }
            Token { kind: Whitespace, text: "\n", range: 26..27 }
            Token { kind: LineComment(Some(Outer)), text: "/// outer doc line", range: 27..45 }
            Token { kind: Whitespace, text: "\n", range: 45..46 }
            Token { kind: LineComment(Some(Inner)), text: "//! inner doc line", range: 46..64 }
            Token { kind: Whitespace, text: "\n", range: 64..65 }
        "#]],
    )
}

#[test]
fn block_comment() {
    check_lexing(
        r"
/* block */
/**/
/*** also block */
/** outer doc block */
/*! inner doc block */
",
        expect![[r#"
            Token { kind: Whitespace, text: "\n", range: 0..1 }
            Token { kind: BlockComment(BlockCommentToken { doc_style: None, terminated: true }), text: "/* block */", range: 1..12 }
            Token { kind: Whitespace, text: "\n", range: 12..13 }
            Token { kind: BlockComment(BlockCommentToken { doc_style: None, terminated: true }), text: "/**/", range: 13..17 }
            Token { kind: Whitespace, text: "\n", range: 17..18 }
            Token { kind: BlockComment(BlockCommentToken { doc_style: None, terminated: true }), text: "/*** also block */", range: 18..36 }
            Token { kind: Whitespace, text: "\n", range: 36..37 }
            Token { kind: BlockComment(BlockCommentToken { doc_style: Some(Outer), terminated: true }), text: "/** outer doc block */", range: 37..59 }
            Token { kind: Whitespace, text: "\n", range: 59..60 }
            Token { kind: BlockComment(BlockCommentToken { doc_style: Some(Inner), terminated: true }), text: "/*! inner doc block */", range: 60..82 }
            Token { kind: Whitespace, text: "\n", range: 82..83 }
        "#]],
    )
}

#[test]
fn nested_block_comments() {
    check_lexing(
        "/* /* */ */'a'",
        expect![[r#"
            Token { kind: BlockComment(BlockCommentToken { doc_style: None, terminated: true }), text: "/* /* */ */", range: 0..11 }
            Token { kind: Literal(Char { terminated: true }), text: "'a'", range: 11..14 }
        "#]],
    )
}
