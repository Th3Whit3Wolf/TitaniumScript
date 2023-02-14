use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod byte_str {
        use super::*;
        #[cfg(test)]
        mod single_line {
            use super::*;

            #[test]
            fn terminated() {
                check(r#"b"a""#, TokenKind::Literal(LiteralKind::ByteStr { terminated: true }))
            }

            #[test]
            fn unterminated() {
                check(r#"b"a"#, TokenKind::Literal(LiteralKind::ByteStr { terminated: false }))
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
    }
}
