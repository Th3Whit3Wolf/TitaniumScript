use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod str {
        use super::*;
        #[cfg(test)]
        mod single_line {
            use super::*;

            #[test]
            fn terminated() {
                check("\"a\"")
            }

            #[test]
            fn unterminated() {
                check(r#""a"#)
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
                )
            }

            #[test]
            fn unterminated() {
                check(
                    r#""a
        "#,
                )
            }
        }
    }
}
