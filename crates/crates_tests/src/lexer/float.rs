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
            fn lower_with_decimal() {
                check(
                    "0b1010.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Binary,
                        empty_exponent: false,
                    }),
                )
            }

            #[test]
            fn upper_with_decimal() {
                check(
                    "0B1010.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Binary,
                        empty_exponent: false,
                    }),
                )
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn lower_with_decimal() {
                    check(
                        "0b1010.01e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Binary,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn upper_with_decimal() {
                    check(
                        "0B1010.01e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Binary,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn lower_without_decimal() {
                    check(
                        "0b1010e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Binary,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn upper_without_decimal() {
                    check(
                        "0B1010e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Binary,
                            empty_exponent: false,
                        }),
                    )
                }
            }
        }

        #[cfg(test)]
        mod decimal {
            use super::*;

            #[test]
            fn with_decimal() {
                check(
                    "200.002",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Decimal,
                        empty_exponent: false,
                    }),
                )
            }

            #[test]
            fn with_negative_prefix() {
                check(
                    "-200.002",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Decimal,
                        empty_exponent: false,
                    }),
                )
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn with_decimal() {
                    check(
                        "200e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Decimal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn without_decimal() {
                    check(
                        "200.002e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Decimal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn with_negative_prefix() {
                    check(
                        "-200.002e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Decimal,
                            empty_exponent: false,
                        }),
                    )
                }
            }
        }

        #[cfg(test)]
        mod octal {
            use super::*;

            #[test]
            fn lower_with_decimal() {
                check(
                    "0o123.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Octal,
                        empty_exponent: false,
                    }),
                )
            }

            #[test]
            fn upper_with_decimal() {
                check(
                    "0O123.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Octal,
                        empty_exponent: false,
                    }),
                )
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn lower_with_decimal() {
                    check(
                        "0o123.01e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Octal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn upper_with_decimal() {
                    check(
                        "0O123.01e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Octal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn lower_without_decimal() {
                    check(
                        "0o123e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Octal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn upper_without_decimal() {
                    check(
                        "0O123e2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Octal,
                            empty_exponent: false,
                        }),
                    )
                }
            }
        }

        #[cfg(test)]
        mod hexadecimal {
            use super::*;

            #[test]
            fn lower_with_decimal() {
                check(
                    "0x123AE.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Hexadecimal,
                        empty_exponent: false,
                    }),
                )
            }

            #[test]
            fn upper_with_decimal() {
                check(
                    "0X123AE.01",
                    TokenKind::Literal(LiteralKind::Float {
                        base: Base::Hexadecimal,
                        empty_exponent: false,
                    }),
                )
            }

            #[cfg(test)]
            mod exponent {
                use super::*;

                #[test]
                fn lower_without_decimal() {
                    check(
                        "0x123AFe2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Hexadecimal,
                            empty_exponent: false,
                        }),
                    )
                }

                #[test]
                fn upper_without_decimal() {
                    check(
                        "0X123AFe2",
                        TokenKind::Literal(LiteralKind::Float {
                            base: Base::Hexadecimal,
                            empty_exponent: false,
                        }),
                    )
                }
            }
        }
    }
}
