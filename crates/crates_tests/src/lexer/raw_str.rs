use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod raw_str {
        use super::*;
        #[cfg(test)]
        mod single_line {
            use super::*;

            #[test]
            fn correct() {
                check("r##\"bar\"##")
            }

            #[test]
            fn no_terminator() {
                check("r###\"bar\"#")
            }

            #[test]
            fn invalid_char() {
                check(r##"#~"abc"#"##)
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
                )
            }

            #[test]
            fn no_terminator() {
                check(r###"r##"abc#"###)
            }

            #[test]
            fn invalid_char() {
                check(
                    r##"#~"a
                    bc"#"##,
                )
            }
        }
    }
}
