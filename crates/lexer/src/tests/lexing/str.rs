use crate::{
    token_kind::{Base, LiteralKind},
    TokenKind,
};

use super::check;

#[cfg(test)]
mod single_line {
    use super::*;

    #[test]
    fn terminated() {
        check("\"a\"", TokenKind::Literal(LiteralKind::Str { terminated: true }))
    }

    #[test]
    fn unterminated() {
        check(r#""a"#, TokenKind::Literal(LiteralKind::Str { terminated: false }))
    }
}

#[cfg(test)]
mod multi_line {
    use super::*;

    #[test]
    fn terminated() {
        check(
            r#""
        a
        ""#,
            TokenKind::Literal(LiteralKind::Str { terminated: true }),
        )
    }

    #[test]
    fn unterminated() {
        check(
            r#""a
        "#,
            TokenKind::Literal(LiteralKind::Str { terminated: false }),
        )
    }
}
