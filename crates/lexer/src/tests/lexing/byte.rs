use crate::{token_kind::LiteralKind, TokenKind};

use super::{check, check_lexing};
use expect_test::expect;

#[cfg(test)]
mod single_line {
    use super::*;

    #[test]
    fn terminated() {
        check("b'a'", TokenKind::Literal(LiteralKind::Byte { terminated: true }))
    }

    #[test]
    fn unterminated() {
        check("b'a", TokenKind::Literal(LiteralKind::Byte { terminated: false }))
    }
}

#[cfg(test)]
mod multi_line {
    use super::*;

    #[test]
    fn terminated() {
        check(
            r#"b'a
'"#,
            TokenKind::Literal(LiteralKind::Byte { terminated: true }),
        )
    }

    #[test]
    fn unterminated() {
        check(
            r#"b'a
        "#,
            TokenKind::Literal(LiteralKind::Byte { terminated: false }),
        )
    }
}

#[test]
fn characters() {
    check_lexing(
        r####"
'a'
b'a'
"####,
        expect![[r#"
        Token { kind: Whitespace, text: "\n", range: 0..1 }
        Token { kind: Literal(Char { terminated: true }), text: "'a'", range: 1..4 }
        Token { kind: Whitespace, text: "\n", range: 4..5 }
        Token { kind: Literal(Byte { terminated: true }), text: "b'a'", range: 5..9 }
        Token { kind: Whitespace, text: "\n", range: 9..10 }
        "#]],
    );
}
