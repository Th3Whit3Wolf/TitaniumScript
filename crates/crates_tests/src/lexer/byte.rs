use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod bytes {
        use super::*;
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
                r#"
                Token { kind: Whitespace, text: "\n        ", range: 0..9 }
                Token { kind: Literal(Char { terminated: true }), text: "'a'", range: 9..12 }
                Token { kind: Whitespace, text: "\n        ", range: 12..21 }
                Token { kind: Literal(Byte { terminated: true }), text: "b'a'", range: 21..25 }
                Token { kind: Whitespace, text: "\n        ", range: 25..34 }
                "#,
            );
        }
    }
}
