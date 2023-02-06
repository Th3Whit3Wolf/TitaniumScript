use crate::{
    token_kind::{Base, LiteralKind},
    TokenKind,
};

use super::check;

#[cfg(test)]
mod binary {
    use super::*;

    #[test]
    fn lower() {
        check(
            "0b1010",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Binary,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn upper() {
        check(
            "0B1010",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Binary,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn empty_lower() {
        check(
            "0b",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Binary,
                empty_int: true,
            }),
        )
    }

    #[test]
    fn empty_upper() {
        check(
            "0B",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Binary,
                empty_int: true,
            }),
        )
    }
}

#[cfg(test)]
mod octal {
    use super::*;

    #[test]
    fn lower() {
        check(
            "0o123",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Octal,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn upper() {
        check(
            "0O123",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Octal,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn empty_lower() {
        check(
            "0o",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Octal,
                empty_int: true,
            }),
        )
    }

    #[test]
    fn empty_upper() {
        check(
            "0O",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Octal,
                empty_int: true,
            }),
        )
    }
}

#[cfg(test)]
mod decimal {
    use super::*;

    #[test]
    fn lower() {
        check(
            "200",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Decimal,
                empty_int: false,
            }),
        )
    }
}

#[cfg(test)]
mod hexadecimal {
    use super::*;

    #[test]
    fn lower() {
        check(
            "0x123AF",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Hexadecimal,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn upper() {
        check(
            "0XAF12321fa",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Hexadecimal,
                empty_int: false,
            }),
        )
    }

    #[test]
    fn empty_lower() {
        check(
            "0x",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Hexadecimal,
                empty_int: true,
            }),
        )
    }

    #[test]
    fn empty_upper() {
        check(
            "0X",
            TokenKind::Literal(LiteralKind::Num {
                base: Base::Hexadecimal,
                empty_int: true,
            }),
        )
    }
}
