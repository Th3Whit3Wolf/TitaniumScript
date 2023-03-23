use logos::{Lexer, Logos};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Logos)]
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
    #[token("/**/", |_| BlockCommentToken{doc_style: None, terminated: true})]
    #[token("/*!", |lex| lex_multiline_comment(lex, Some(DocStyle::Inner)))]
    #[token("/**", |lex| lex_multiline_comment(lex, Some(DocStyle::Outer)))]
    #[token("/***", |lex| lex_multiline_comment(lex, None))]
    #[token("/*", |lex| lex_multiline_comment(lex, None))]
    BlockComment(BlockCommentToken),

    /// Any whitespace character sequence.
    #[regex("[\t\r \u{0009}\u{000A}\u{000B}\u{000C}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}\n]+")]
    Whitespace,

    /// "ident" or "continue"
    ///
    /// At this step, keywords are also considered identifiers.
    #[regex(r"([_]|\p{XID_Start})\p{XID_Continue}*")]
    Ident,

    /// "r#ident"
    #[regex(r"r#([_]|\p{XID_Start})\p{XID_Continue}*")]
    RawIdent,

    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    ///
    #[regex("0b[_]?", |_| LiteralKind::Num { base: Base::Binary, empty_int: true })]
    #[regex("0o[_]?", |_| LiteralKind::Num { base: Base::Octal, empty_int: true })]
    #[regex("0x[_]?", |_| LiteralKind::Num { base: Base::Hexadecimal, empty_int: true })]
    #[regex("0b[0-9]+[0-9_]*", |lex| LiteralKind::lex_num(lex, Base::Binary))]
    #[regex("0o[0-9]+[0-9_]*", |lex| LiteralKind::lex_num(lex, Base::Octal))]
    #[regex("0x[0-9a-fA-F]+[0-9a-fA-F_]*", |lex| LiteralKind::lex_num(lex, Base::Hexadecimal) )]
    #[regex("[0-9][0-9_]*", |lex| LiteralKind::lex_num(lex, Base::Decimal) )]
    #[regex("b'|'", LiteralKind::lex_single_quote)]
    #[regex("b\"|\"", LiteralKind::lex_double_quote)]
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
    #[token("return", |_| Keywords::Return)]
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
}

impl LiteralKind {
    fn lex_num(lex: &mut Lexer<TokenKind>, base: Base) -> Self {
        let remaining = lex.remainder();
        let first_char = remaining.chars().nth(0);
        let second_char = remaining.chars().nth(1);

        if let Some(first) = first_char {
            match first {
                // Don't be greedy if this is actually an
                // integer literal followed by field/method access or a range pattern
                // (`0..2` and `12.foo()`)
                '.' => {
                    if let Some(second) = second_char {
                        if second != '.' && !is_id_start(second) {
                            lex.bump(1);
                            let mut empty_exponent = false;
                            if second.is_digit(10) {
                                eat_decimal_digits(lex);
                                if let Some(last_char) = lex.remainder().chars().nth(0) {
                                    if last_char == 'e' || last_char == 'E' {
                                        lex.bump(1);
                                        empty_exponent = !eat_float_exponent(lex);
                                    }
                                }
                            }
                            LiteralKind::Float { base, empty_exponent }
                        } else {
                            LiteralKind::Num { base, empty_int: false }
                        }
                    } else {
                        LiteralKind::Float { base, empty_exponent: false }
                    }
                }
                'e' | 'E' => {
                    lex.bump(1);
                    let empty_exponent = !eat_float_exponent(lex);
                    LiteralKind::Float { base, empty_exponent }
                }
                _ => LiteralKind::Num { base, empty_int: false },
            }
        } else {
            LiteralKind::Num { base, empty_int: false }
        }
    }

    fn lex_single_quote(lex: &mut Lexer<TokenKind>) -> Self {
        let is_char = lex.slice().len() == 1;
        let terminated = single_quoted_string(lex);
        if is_char {
            LiteralKind::Char { terminated }
        } else {
            LiteralKind::Byte { terminated }
        }
    }

    fn lex_double_quote(lex: &mut Lexer<TokenKind>) -> Self {
        let is_str = lex.slice().len() == 1;

        let terminated = double_quoted_string(lex);
        if is_str {
            LiteralKind::Str { terminated }
        } else {
            return LiteralKind::ByteStr { terminated };
        }
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
    Return,
    SelfLower,
    SelfUpper,
    Trait,
    True,
    Type,
    Where,
    While,
}

fn lex_multiline_comment(
    lex: &mut Lexer<TokenKind>,
    doc_style: Option<DocStyle>,
) -> BlockCommentToken {
    let mut depth = 1_usize;
    let mut last_char = 0_u8;

    for (i, b) in lex.remainder().bytes().enumerate() {
        match (last_char, b) {
            (b'/', b'*') => {
                depth += 1;
            }
            (b'*', b'/') => {
                depth -= 1;
            }
            _ => (),
        }
        last_char = b;

        if depth == 0 {
            lex.bump(i + 1);
            return BlockCommentToken { doc_style, terminated: true };
        }
    }

    lex.bump(lex.remainder().bytes().len());
    BlockCommentToken { doc_style, terminated: false }
}

/// Eats double-quoted string and returns true
/// if string is terminated.
fn double_quoted_string(lex: &mut Lexer<TokenKind>) -> bool {
    let mut last_char = b'"';

    for (i, b) in lex.remainder().bytes().enumerate() {
        if b == b'"' && last_char != b'\\' && !(i == 1 && last_char == b'b') {
            lex.bump(i + 1);
            return true;
        }
        last_char = b;
    }
    lex.bump(lex.remainder().bytes().len());
    false
}

/// Eats single-quoted string and returns true
/// if string is terminated.
fn single_quoted_string(lex: &mut Lexer<TokenKind>) -> bool {
    let mut last_char = 0_u8;

    for (i, b) in lex.remainder().bytes().enumerate() {
        if b == b'\'' && last_char != b'\\' && !(i == 1 && last_char == b'b') {
            lex.bump(i + 1);
            return true;
        }

        if last_char == b'\n' {
            lex.bump(i - 1);
            return false;
        }

        if last_char == b'/' {
            lex.bump(i + 1);
            return false;
        }

        if b == b'\\' && last_char == b'\\' {
            last_char = 0_u8;
        } else {
            last_char = b;
        }
    }
    lex.bump(lex.remainder().bytes().len());
    false
}

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
    // This is XID_Start OR '_' (which formally is not a XID_Start).
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

fn eat_decimal_digits(lex: &mut Lexer<TokenKind>) -> bool {
    let mut has_digits = false;
    let mut bmp = 0;
    for b in lex.remainder().bytes() {
        match b {
            b'_' => {
                bmp += 1;
            }
            b'0'..=b'9' => {
                bmp += 1;
                has_digits = true;
            }
            _ => {
                break;
            }
        }
    }

    lex.bump(bmp);
    has_digits
}

fn eat_float_exponent(lex: &mut Lexer<TokenKind>) -> bool {
    let first = lex.remainder().chars().next();
    if first == Some('-') || first == Some('+') {
        lex.bump(1);
    }
    eat_decimal_digits(lex)
}
