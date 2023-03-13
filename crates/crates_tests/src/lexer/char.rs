use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod chars {
        use super::*;
        #[cfg(test)]
        mod single_line {
            use super::*;

            #[test]
            fn terminated() {
                check(r#"'a'"#)
            }

            #[test]
            fn unterminated() {
                check(r#"' a "#)
            }
        }

        #[test]
        fn characters() {
            check_lexing(
                "'a' ' ' '\\n'",
                r#"
            Token { kind: Literal(Char { terminated: true }), text: "'a'", range: 0..3 }
            Token { kind: Whitespace, text: " ", range: 3..4 }
            Token { kind: Literal(Char { terminated: true }), text: "' '", range: 4..7 }
            Token { kind: Whitespace, text: " ", range: 7..8 }
            Token { kind: Literal(Char { terminated: true }), text: "'\\n'", range: 8..12 }
        "#,
            );
        }
    }
}
