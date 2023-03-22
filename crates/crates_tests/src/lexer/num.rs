use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod number {
        use super::*;

        #[cfg(test)]
        mod binary {
            use super::*;

            #[test]
            fn valid() {
                check("0b1010")
            }

            #[test]
            fn empty() {
                check("0b")
            }
        }

        #[cfg(test)]
        mod octal {
            use super::*;

            #[test]
            fn valid() {
                check("0o123")
            }

            #[test]
            fn empty() {
                check("0o")
            }
        }

        #[cfg(test)]
        mod decimal {
            use super::*;

            #[test]
            fn lower() {
                check("200")
            }

            #[test]
            fn range() {
                check("0..2")
            }
        }

        #[cfg(test)]
        mod hexadecimal {
            use super::*;

            #[test]
            fn valid() {
                check("0x123AF")
            }

            #[test]
            fn empty() {
                check("0x")
            }
        }
    }
}
