use super::*;
#[cfg(test)]
mod literal {
    use super::*;
    #[cfg(test)]
    mod float {
        use super::*;

        #[cfg(test)]
        mod binary {
            use super::*;

            #[test]
            fn with_decimal() {
                check("0b1010.01")
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn with_decimal() {
                    check("0b1010.01e2")
                }

                #[test]
                fn without_decimal() {
                    check("0b1010e2")
                }
            }
        }

        #[cfg(test)]
        mod decimal {
            use super::*;

            #[test]
            fn with_decimal() {
                check("200.002")
            }
            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn with_decimal() {
                    check("200.00e2")
                }

                #[test]
                fn without_decimal() {
                    check("200e2")
                }
            }
        }

        #[cfg(test)]
        mod octal {
            use super::*;

            #[test]
            fn with_decimal() {
                check("0o123.01")
            }
            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn with_decimal() {
                    check("0o123.01e2")
                }

                #[test]
                fn without_decimal() {
                    check("0o123e2")
                }
            }
        }

        #[cfg(test)]
        mod hexadecimal {
            use super::*;

            #[test]
            fn with_decimal() {
                check("0x123AE.01")
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn valid() {
                    check("0x123A.01e2")
                }
            }
        }
    }
}
