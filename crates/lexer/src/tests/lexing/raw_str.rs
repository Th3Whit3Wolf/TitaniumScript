use crate::{token_kind::LiteralKind, TokenKind};

use super::check;

#[cfg(test)]
mod single_line {
    use super::*;

    #[test]
    fn correct() {
        check(
            "r##\"bar\"##",
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn no_terminator() {
        check(
            "r###\"bar\"#",
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 3,
                n_end_hashes: 1,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn invalid_char() {
        check(
            "r#a#\"bar\"##",
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: Some('a'),
            }),
        )
    }
}

#[cfg(test)]
mod multi_line {
    use super::*;

    #[test]
    fn correct() {
        check(
            r####"r##"
                abc
            "##"####,
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn no_terminator() {
        check(
            r####"r###"
                abc
            "#
"####,
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 3,
                n_end_hashes: 1,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn invalid_char() {
        check(
            r####"r#a#"
                abc
            "##"####,
            TokenKind::Literal(LiteralKind::RawStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: Some('a'),
            }),
        )
    }
}
