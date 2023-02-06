use crate::{token_kind::LiteralKind, TokenKind};

use super::check;

#[cfg(test)]
mod single_line {
    use super::*;

    #[test]
    fn terminated() {
        check(
            r#"b"a""#,
            TokenKind::Literal(LiteralKind::ByteStr { terminated: true }),
        )
    }

    #[test]
    fn unterminated() {
        let str = r#"b"a"#;
        check(
            r#"b"a"#,
            TokenKind::Literal(LiteralKind::ByteStr { terminated: false }),
        )
    }
}

#[cfg(test)]
mod multi_line {
    use super::*;

    #[test]
    fn terminated() {
        check(
            r#"b"
        a
        ""#,
            TokenKind::Literal(LiteralKind::ByteStr { terminated: true }),
        )
    }

    #[test]
    fn unterminated() {
        check(
            r#"b"a
        "#,
            TokenKind::Literal(LiteralKind::ByteStr { terminated: false }),
        )
    }
}
