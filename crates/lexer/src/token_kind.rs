use logos::internal::LexerInternal;
use logos::{Lexer, Logos};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Logos)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern octal = r"[0-7][_0-7]*")]
#[logos(subpattern binary = r"[0-1][_0-1]*")]
#[logos(subpattern exp = r"[eE][+-]?[0-9][_0-9]*")]
#[logos(subpattern nexp = r"([eE][+-]?)|([eE][+-]?([A-Za-z_]+[A-Za-z0-9_]*))")]
#[logos(subpattern empty_int = r"[A-Za-z_]+[A-Za-z0-9_]*")]
#[logos(subpattern ident = r"[_]?([A-Za-z0-9_]|\p{XID_Start}\p{XID_Continue})*")]

pub enum TokenKind {
    // Multi-char tokens:
    /// "// comment"
    #[regex("//[^\n]*", |_| None as Option<DocStyle> )]
    #[regex("////[^\n]*", |_| None as Option<DocStyle> )]
    #[regex("///[^\n]*", |_| Some(DocStyle::Outer) )]
    #[regex("//![^\n]*", |_| Some(DocStyle::Inner) )]
    LineComment(Option<DocStyle>),

    /// `/* block comment */`
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    #[token("/*", BlockCommentToken::from_lex)]
    #[token("/*!", BlockCommentToken::from_lex)]
    #[token("/**", BlockCommentToken::from_lex)]
    BlockComment(BlockCommentToken),

    /// Any whitespace character sequence.
    #[token(r"\u{0009}")] // \t
    #[token(r"\u{000A}")] // \n
    #[token(r"\u{000B}")] // vertical tab
    #[token(r"\u{000C}")] // form feed
    #[token(r"\u{0085}")] // NEXT LINE from latin1
    #[token(r"\u{200E}")] // LEFT-TO-RIGHT MARK
    #[token(r"\u{200F}")] // RIGHT-TO-LEFT MARK
    #[token(r"\u{2028}")] // LINE SEPARATOR
    #[token(r"\u{2029}")] // PARAGRAPH SEPARATOR
    #[regex("[\t\r \u{0009}\u{000A}\u{000B}\u{000C}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}\n]+")]
    Whitespace,

    /// "ident" or "continue"
    ///
    /// At this step, keywords are also considered identifiers.
    #[regex(r"(?&ident)")]
    Ident,

    /// "r#ident"
    #[regex(r"r#\p{XID_Start}\p{XID_Continue}*")]
    RawIdent,

    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    ///
    #[regex("0[bB](?&binary)", |_|  LiteralKind::Num { base: Base::Binary, empty_int: false })]
    #[regex("0b(?&empty_int)?", |_|  LiteralKind::Num { base: Base::Binary, empty_int: true })]
    #[regex("0B(?&empty_int)?", |_|  LiteralKind::Num { base: Base::Binary, empty_int: true })]
    #[regex("0[oO](?&octal)", |_|  LiteralKind::Num { base: Base::Octal, empty_int: false })]
    #[regex("0o(?&empty_int)?", |_|  LiteralKind::Num { base: Base::Octal, empty_int: true })]
    #[regex("0O(?&empty_int)?", |_|  LiteralKind::Num { base: Base::Octal, empty_int: true })]
    #[regex("(?&decimal)", |_|  LiteralKind::Num { base: Base::Decimal, empty_int: false })]
    #[regex("0[xX](?&hex)", |_|  LiteralKind::Num { base: Base::Hexadecimal, empty_int: false })]
    #[regex("0x([G-Zg-z_]+[A-Za-z0-9_]*)?", |_|  LiteralKind::Num { base: Base::Hexadecimal, empty_int: true })]
    #[regex("0X([G-Zg-z_]+[A-Za-z0-9_]*)?", |_|  LiteralKind::Num { base: Base::Hexadecimal, empty_int: true })]
    #[regex(r#"0[bB](((?&binary)\.(?&decimal)[eE][+-]?)|((?&binary)[eE][+-]?))"#, |_| LiteralKind::Float { base: Base::Binary, empty_exponent: true})]
    #[regex(r#"0[bB](((?&binary)\.(?&decimal)(?&exp))|(?&binary)\.(?&decimal)|((?&binary)(?&exp)))"#, |_| LiteralKind::Float { base: Base::Binary, empty_exponent: false})]
    #[regex(r#"0[oO](((?&octal)\.(?&decimal)[eE][+-]?)|((?&octal)[eE][+-]?))"#, |_| LiteralKind::Float { base: Base::Octal, empty_exponent: true})]
    #[regex(r#"0[oO](((?&octal)\.(?&decimal)(?&exp))|(?&octal)\.(?&decimal)|((?&octal)(?&exp)))"#, |_| LiteralKind::Float { base: Base::Octal, empty_exponent: false})]
    #[regex(r#"[-]?(((?&decimal)\.(?&decimal)(?&nexp))|((?&decimal)(?&nexp)))"#, |_| LiteralKind::Float { base: Base::Decimal, empty_exponent: true})]
    #[regex(r#"[-]?(((?&decimal)\.(?&decimal)(?&exp))|(?&decimal)\.(?&decimal)|((?&decimal)(?&exp)))"#, |_| LiteralKind::Float { base: Base::Decimal, empty_exponent: false})]
    #[regex(r#"0[xX](((?&hex)\.(?&decimal)[eE][+-]?)|((?&hex)[eE][+-]?))"#, |_| LiteralKind::Float { base: Base::Hexadecimal, empty_exponent: true})]
    #[regex(r#"0[xX](((?&hex)\.(?&decimal)(?&exp))|(?&hex)\.(?&decimal)|((?&hex)(?&exp)))"#, |_| LiteralKind::Float { base: Base::Hexadecimal, empty_exponent: false})]
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\"|\\)*["]?"#, LiteralKind::lex_str)]
    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\')*[']?"#, LiteralKind::lex_char)]
    #[regex(r#"b'([^'\\]|\\t|\\u|\\n|\\')*[']?"#, LiteralKind::lex_byte)]
    #[regex(r#"b"([^"\\]|\\t|\\u|\\n|\\")*["]?"#, LiteralKind::lex_byte_str)]
    #[regex(r#"r[\w]*#*[\w]*#*""#, LiteralKind::lex_raw_str)]
    #[regex(r#"br[\w]*#[\w]*#*""#, LiteralKind::lex_raw_byte_str)]
    Literal(LiteralKind),

    #[token("as", |_| Keywords::As)]
    #[token("async", |_| Keywords::Async)]
    #[token("await", |_| Keywords::Await)]
    #[token("break", |_| Keywords::Break)]
    #[token("const", |_| Keywords::Const)]
    #[token("continue", |_| Keywords::Continue)]
    #[token("else", |_| Keywords::Else)]
    #[token("enum", |_| Keywords::Enum)]
    #[token("false", |_| Keywords::False)]
    #[token("fn", |_| Keywords::Fn)]
    #[token("for", |_| Keywords::For)]
    #[token("if", |_| Keywords::If)]
    #[token("impl", |_| Keywords::Impl)]
    #[token("in", |_| Keywords::In)]
    #[token("let", |_| Keywords::Let)]
    #[token("loop", |_| Keywords::Loop)]
    #[token("match", |_| Keywords::Match)]
    #[token("self", |_| Keywords::SelfLower)]
    #[token("Self", |_| Keywords::SelfUpper)]
    #[token("trait", |_| Keywords::Trait)]
    #[token("true", |_| Keywords::True)]
    #[token("type", |_| Keywords::Type)]
    #[token("where", |_| Keywords::Where)]
    #[token("while", |_| Keywords::While)]
    Keyword(Keywords),

    // One-char tokens:
    /// ";"
    #[token(";")]
    Semi,
    /// ","
    #[token(",")]
    Comma,
    /// "."
    #[token(".")]
    Dot,
    /// "("
    #[token("(")]
    OpenParen,
    /// ")"
    #[token(")")]
    CloseParen,
    /// "{"
    #[token("{")]
    OpenBrace,
    /// "}"
    #[token("}")]
    CloseBrace,
    /// "["
    #[token("[")]
    OpenBracket,
    /// "]"
    #[token("]")]
    CloseBracket,
    /// "@"
    #[token("@")]
    At,
    /// "#"
    #[token("#")]
    Pound,
    /// "~"
    #[token("~")]
    Tilde,
    /// "?"
    #[token("?")]
    Question,
    /// ";"
    #[token(":")]
    Colon,
    /// "$"
    #[token("$")]
    Dollar,
    /// "="
    #[token("=")]
    Eq,
    /// "!"
    #[token("!")]
    Bang,
    /// "<"
    #[token("<")]
    Lt,
    /// ">"
    #[token(">")]
    Gt,
    /// "-"
    #[token("-")]
    Minus,
    #[token("_")]
    Underscore,
    /// "&"
    #[token("&")]
    And,
    /// "|"
    #[token("|")]
    Or,
    /// "+"
    #[token("+")]
    Plus,
    /// "*"
    #[token("*")]
    Star,
    /// "/"
    #[token("/")]
    Slash,
    /// "^"
    #[token("^")]
    Caret,
    /// "%"
    #[token("%")]
    Percent,
    #[error]
    Unknown,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment(_) | Self::BlockComment(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockCommentToken {
    pub doc_style: Option<DocStyle>,
    pub terminated: bool,
}

impl BlockCommentToken {
    fn from_lex(lex: &mut Lexer<TokenKind>) -> Self {
        let slice = lex.slice().as_bytes();

        let doc_style: Option<DocStyle> = match slice {
            b"/*!" => Some(DocStyle::Inner),
            b"/**" => {
                if let Some(next) = lex.read::<u8>() {
                    lex.bump(1);
                    if next == b'/' {
                        return BlockCommentToken { doc_style: None, terminated: true };
                    } else if next != b'*' {
                        Some(DocStyle::Outer)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        let mut depth = 1usize;
        let mut terminated = false;

        loop {
            let r = lex.read::<u8>();
            if let Some(read) = r {
                lex.bump(1);
                match read {
                    b'/' => {
                        if let Some(asterisk) = lex.read::<u8>() {
                            if asterisk == b'*' {
                                depth += 1;
                            }
                        } else {
                            break;
                        }
                    }
                    b'*' => {
                        if let Some(slash) = lex.read::<u8>() {
                            if slash == b'/' {
                                lex.bump(1);
                                depth -= 1;

                                if depth == 0 {
                                    // This block comment is closed, so for a construction like "/* */ */"
                                    // there will be a successfully parsed block comment "/* */"
                                    // and " */" will be processed separately.
                                    terminated = true;
                                    break;
                                }
                            }
                            lex.bump(1);
                        } else {
                            break;
                        }
                    }
                    0 => break,
                    // None => break,
                    _ => (),
                }
            } else {
                break;
            }
        }

        BlockCommentToken { doc_style, terminated }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocStyle {
    Outer,
    Inner,
}

// Note that the suffix is *not* considered when deciding the `LiteralKind` in
// this type. This means that float literals like `1f32` are classified by this
// type as `Int`. (Compare against `rustc_ast::token::LitKind` and
// `rustc_ast::ast::LitKind.)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// "12_u8", "0o100", "0b120i99", "1f32".
    Num { base: Base, empty_int: bool },
    /// "12.34f32", "1e3", but not "1f32`.
    Float { base: Base, empty_exponent: bool },
    /// "'a'", "'\\'", "'''", "';"
    Char { terminated: bool },
    /// "b'a'", "b'\\'", "b'''", "b';"
    Byte { terminated: bool },
    /// ""abc"", ""abc"
    Str { terminated: bool },
    /// "b"abc"", "b"abc"
    ByteStr { terminated: bool },
    /// "r"abc"", "r#"abc"#", "r####"ab"###"c"####", "r#"a". `None` indicates
    /// an invalid literal.
    RawStr { n_start_hashes: u32, n_end_hashes: u32, bad_char: Option<char> },
    /// "br"abc"", "br#"abc"#", "br####"ab"###"c"####", "br#"a". `None`
    /// indicates an invalid literal.
    RawByteStr { n_start_hashes: u32, n_end_hashes: u32, bad_char: Option<char> },
}

impl LiteralKind {
    fn lex_str(lex: &mut Lexer<TokenKind>) -> Self {
        let terminated = double_quoted_string(lex);

        LiteralKind::Str { terminated }
    }
    fn lex_char(lex: &mut Lexer<TokenKind>) -> Self {
        let terminated = single_quoted_string(lex);

        LiteralKind::Char { terminated }
    }

    fn lex_byte(lex: &mut Lexer<TokenKind>) -> Self {
        let terminated = single_quoted_string(lex);

        LiteralKind::Byte { terminated }
    }

    fn lex_byte_str(lex: &mut Lexer<TokenKind>) -> Self {
        let terminated = double_quoted_string(lex);

        LiteralKind::ByteStr { terminated }
    }

    fn lex_raw_str(lex: &mut Lexer<TokenKind>) -> Self {
        let (n_start_hashes, n_end_hashes, bad_char) = raw_string(lex);

        LiteralKind::RawStr { n_start_hashes, n_end_hashes, bad_char }
    }

    fn lex_raw_byte_str(lex: &mut Lexer<TokenKind>) -> Self {
        let (n_start_hashes, n_end_hashes, bad_char) = raw_string(lex);

        LiteralKind::RawByteStr { n_start_hashes, n_end_hashes, bad_char }
    }
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with "0b".
    Binary = 2,
    /// Literal starts with "0o".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keywords {
    As,
    Async,
    Await,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    SelfLower,
    SelfUpper,
    Trait,
    True,
    Type,
    Where,
    While,
}

/// Eats double-quoted string and returns true
/// if string is terminated.
fn double_quoted_string(lex: &mut Lexer<TokenKind>) -> bool {
    // lex.bump(1);
    // println!("Slice: {}", lex.slice());
    // while let Some(read) = lex.read::<u8>() {
    //     let hint = lex.size_hint();
    //     if hint == (0, None) {
    //         break;
    //     }

    //     lex.bump(1);

    //     match read {
    //         // Quotes are terminated, finish parsing.
    //         b'"' => {
    //             return true;
    //         }
    //         // Escaped slash is considered one character, so bump twice.
    //         b'\\' => {
    //             if let Some(c) = lex.read::<u8>() {
    //                 if c == b'\\' || c == b'"' {
    //                     lex.bump(1);
    //                 }
    //             }
    //         }
    //         0 => break,
    //         // Skip the character.
    //         _ => (),
    //     }
    // }
    // // String was not terminated.
    // false
    lex.slice().ends_with('"')
}

/// Eats single-quoted string and returns true
/// if string is terminated.
fn single_quoted_string(lex: &mut Lexer<TokenKind>) -> bool {
    // println!("Slice: {}", lex.slice());
    // lex.bump(1);
    // println!("Slice: {}", lex.slice());

    // while let Some(read) = lex.read::<u8>() {
    //     let hint = lex.size_hint();
    //     if hint == (0, None) {
    //         break;
    //     }

    //     lex.bump(1);
    //     match read {
    //         // Quotes are terminated, finish parsing.
    //         b'\'' => {
    //             return true;
    //         }
    //         // Probably beginning of the comment, which we don't want to include
    //         // to the error report.
    //         b'/' => {
    //             break;
    //         }
    //         // Newline without following '\'' means unclosed quote, stop parsing.
    //         b'\n' => {
    //             if let Some(c) = lex.read::<u8>() {
    //                 if c == b'\'' {
    //                     lex.bump(1);
    //                     return true;
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //         // Escaped slash is considered one character, so bump twice.
    //         b'\\' => {
    //             lex.bump(1);
    //         }
    //         0 => break,
    //         // Skip the character.
    //         _ => (),
    //     }
    // }
    // // String was not terminated.
    // false
    lex.slice().ends_with('\'')
}

fn raw_string(lex: &mut Lexer<TokenKind>) -> (u32, u32, Option<char>) {
    let mut n_start_hashes: u32 = 0;
    let mut n_end_hashes: u32 = 0;
    let mut bad_char: Option<char> = None;
    let slice = lex.slice();

    let chars = if slice.starts_with("br") {
        slice[2..slice.len() - 1].chars()
    } else {
        slice[1..slice.len() - 1].chars()
    };

    for char in chars {
        if char == '#' {
            n_start_hashes += 1;
        } else if bad_char.is_none() {
            bad_char = Some(char)
        }
    }

    let mut start = false;

    loop {
        let r = lex.read::<u8>();

        if let Some(read) = r {
            lex.bump(1);
            match read {
                // Quotes are terminated, finish parsing.
                b'"' => start = true,
                // Escaped slash is considered one character, so bump twice.
                b'#' => {
                    if start && n_end_hashes < n_start_hashes {
                        n_end_hashes += 1
                    }
                }
                0 => break,
                // Skip the character.
                _ => {
                    if start {
                        // Reset start
                        start = false
                    }
                }
            }
        } else {
            break;
        }
        if n_start_hashes > 0 && n_end_hashes == n_start_hashes {
            break;
        }
    }

    (n_start_hashes, n_end_hashes, bad_char)
}
