use crate::{
    token_kind::{Base, LiteralKind},
    TokenKind,
};

use super::check;

#[cfg(test)]
mod single_line {
    use super::*;

    #[test]
    fn correct() {
        check(
            "br##\"bar\"##",
            TokenKind::Literal(LiteralKind::RawByteStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn no_terminator() {
        check(
            "br###\"bar\"#",
            TokenKind::Literal(LiteralKind::RawByteStr {
                n_start_hashes: 3,
                n_end_hashes: 1,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn invalid_char() {
        check(
            "br#a#\"bar\"##",
            TokenKind::Literal(LiteralKind::RawByteStr {
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
            r####"br##"
                abc
            "##"####,
            TokenKind::Literal(LiteralKind::RawByteStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn no_terminator() {
        check(
            r####"br###"
                abc
            "#
"####,
            TokenKind::Literal(LiteralKind::RawByteStr {
                n_start_hashes: 3,
                n_end_hashes: 1,
                bad_char: None,
            }),
        )
    }

    #[test]
    fn invalid_char() {
        check(
            r####"br#a#"
                abc
            "##"####,
            TokenKind::Literal(LiteralKind::RawByteStr {
                n_start_hashes: 2,
                n_end_hashes: 2,
                bad_char: Some('a'),
            }),
        )
    }
}
