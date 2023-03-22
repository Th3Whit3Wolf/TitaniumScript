use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod raw_byte_str {
        use super::*;
        #[cfg(test)]
        mod single_line {
            use super::*;

            #[test]
            fn correct() {
                check("br##\"bar\"##")
            }

            #[test]
            fn empty() {
                check("br\"\" ")
            }

            #[test]
            fn no_terminator() {
                check("br###\"bar\"#")
            }

            #[test]
            fn invalid_char() {
                check(r##"br#~"abc"#"##)
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
                )
            }

            #[test]
            fn no_terminator() {
                check(
                    r###"br##"a
bc#"###,
                )
            }

            #[test]
            fn invalid_char() {
                check(
                    r####"r##"#~"a
                    bc"#"##"####,
                )
            }
        }
    }
}
