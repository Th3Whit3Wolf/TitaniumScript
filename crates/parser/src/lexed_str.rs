//! Lexing `&str` into a sequence of Rust tokens.
//!
//! Note that strictly speaking the parser in this crate is not required to work
//! on tokens which originated from text. Macros, eg, can synthesize tokens out
//! of thin air. So, ideally, lexer should be an orthogonal crate. It is however
//! convenient to include a text-based lexer here!
//!
//! Note that these tokens, unlike the tokens we feed into the parser, do
//! include info about comments and whitespace.

use std::ops;

use crate::{
    SyntaxKind::{self, *},
    T,
};

use lexer::{BlockCommentToken, Lexer, LiteralKind, Token};
pub struct LexedStr<'a> {
    text: &'a str,
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    token: u32,
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &'a str) -> LexedStr<'a> {
        let mut conv = Converter::new(text);
        // if let Some(shebang_len) = lexer::strip_shebang(text) {
        //     conv.res.push(SHEBANG, conv.offset);
        //     conv.offset = shebang_len;
        // };

        // while let Some(token) = Lexer::new(text).next() {
        //     dbg!("{:?}", &token);
        //     let token_text = token.text;
        //     conv.extend_token(&token.kind, token_text);
        // }

        while let Some(token) = Lexer::new(&text[conv.offset..]).next() {
            dbg!("{:?}", &token);
            let token_text = token.text;
            conv.extend_token(&token.kind, token_text);
        }

        // for token in lexer::tokenize(&text[conv.offset..]) {
        //     let token_text = &text[conv.offset..][..token.len];

        //     conv.extend_token(&token.kind, token_text);
        // }

        conv.finalize_with_eof()
    }

    pub fn single_token(text: &'a str) -> Option<(SyntaxKind, Option<String>)> {
        if text.is_empty() {
            return None;
        }

        if let Some(token) = Lexer::new(text).next() {
            if usize::from(token.range.end() - token.range.start()) != text.len() {
                return None;
            }

            let mut conv = Converter::new(text);
            conv.extend_token(&token.kind, text);
            match &*conv.res.kind {
                [kind] => Some((*kind, conv.res.error.pop().map(|it| it.msg))),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
    }
    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        r.end - r.start
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self.error.binary_search_by_key(&(i as u32), |i| i.token).ok()?;
        Some(self.error[err].msg.as_str())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token as usize, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
}

impl<'a> Converter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            res: LexedStr { text, kind: Vec::new(), start: Vec::new(), error: Vec::new() },
            offset: 0,
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(EOF, self.offset);
        self.res
    }

    fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<&str>) {
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some(err) = err {
            let token = self.res.len() as u32;
            let msg = err.to_string();
            self.res.error.push(LexError { msg, token });
        }
    }

    fn extend_token(&mut self, kind: &lexer::TokenKind, token_text: &str) {
        // A note on an intended tradeoff:
        // We drop some useful information here (see patterns with double dots `..`)
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let mut err = "";

        let syntax_kind = {
            match kind {
                lexer::TokenKind::LineComment(_) => COMMENT,
                lexer::TokenKind::BlockComment(BlockCommentToken { doc_style: _, terminated }) => {
                    if !terminated {
                        err = "Missing trailing `*/` symbols to terminate the block comment";
                    }
                    COMMENT
                }

                lexer::TokenKind::Whitespace => WHITESPACE,
                lexer::TokenKind::Underscore => UNDERSCORE,

                lexer::TokenKind::Ident => SyntaxKind::from_keyword(token_text).unwrap_or(IDENT),

                lexer::TokenKind::RawIdent => IDENT,
                lexer::TokenKind::Literal(kind) => {
                    self.extend_literal(token_text.len(), kind);
                    return;
                }

                // lexer::TokenKind::Lifetime { starts_with_number } => {
                //     if *starts_with_number {
                //         err = "Lifetime name cannot start with a number";
                //     }
                //     // LIFETIME_IDENT
                //     IDENT
                // }
                lexer::TokenKind::Semi => T![;],
                lexer::TokenKind::Comma => T![,],
                lexer::TokenKind::Dot => T![.],
                lexer::TokenKind::OpenParen => T!['('],
                lexer::TokenKind::CloseParen => T![')'],
                lexer::TokenKind::OpenBrace => T!['{'],
                lexer::TokenKind::CloseBrace => T!['}'],
                lexer::TokenKind::OpenBracket => T!['['],
                lexer::TokenKind::CloseBracket => T![']'],
                lexer::TokenKind::At => T![@],
                lexer::TokenKind::Pound => T![#],
                lexer::TokenKind::Tilde => T![~],
                lexer::TokenKind::Question => T![?],
                lexer::TokenKind::Colon => T![:],
                lexer::TokenKind::Dollar => T![$],
                lexer::TokenKind::Eq => T![=],
                lexer::TokenKind::Bang => T![!],
                lexer::TokenKind::Lt => T![<],
                lexer::TokenKind::Gt => T![>],
                lexer::TokenKind::Minus => T![-],
                lexer::TokenKind::And => T![&],
                lexer::TokenKind::Or => T![|],
                lexer::TokenKind::Plus => T![+],
                lexer::TokenKind::Star => T![*],
                lexer::TokenKind::Slash => T![/],
                lexer::TokenKind::Caret => T![^],
                lexer::TokenKind::Percent => T![%],
                lexer::TokenKind::Unknown => ERROR,
                lexer::TokenKind::Keyword(kw) => match kw {
                    lexer::Keywords::As => AS_KW,
                    lexer::Keywords::Async => ASYNC_KW,
                    lexer::Keywords::Await => AWAIT_KW,
                    lexer::Keywords::Break => BREAK_KW,
                    lexer::Keywords::Const => CONST_KW,
                    lexer::Keywords::Continue => CONTINUE_KW,
                    lexer::Keywords::Else => ELSE_KW,
                    lexer::Keywords::Enum => ENUM_KW,
                    lexer::Keywords::False => FALSE_KW,
                    lexer::Keywords::Fn => FN_KW,
                    lexer::Keywords::For => FOR_KW,
                    lexer::Keywords::If => IF_KW,
                    lexer::Keywords::Impl => IMPL_KW,
                    lexer::Keywords::In => IN_KW,
                    lexer::Keywords::Let => LET_KW,
                    lexer::Keywords::Loop => LOOP_KW,
                    lexer::Keywords::Match => MATCH_KW,
                    lexer::Keywords::SelfLower => SELF_KW,
                    lexer::Keywords::SelfUpper => SELF_TYPE_KW,
                    lexer::Keywords::Trait => TRAIT_KW,
                    lexer::Keywords::True => TRUE_KW,
                    lexer::Keywords::Type => TYPE_KW,
                    lexer::Keywords::Where => WHERE_KW,
                    lexer::Keywords::While => WHILE_KW,
                },
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, token_text.len(), err);
    }

    fn extend_literal(&mut self, len: usize, kind: &lexer::LiteralKind) {
        let mut err = "";

        let syntax_kind = match *kind {
            lexer::LiteralKind::Num { empty_int, base: _ } => {
                if empty_int {
                    err = "Missing digits after the integer base prefix";
                }
                INT_NUMBER
            }
            lexer::LiteralKind::Float { empty_exponent, base: _ } => {
                if empty_exponent {
                    err = "Missing digits after the exponent symbol";
                }
                FLOAT_NUMBER
            }
            lexer::LiteralKind::Char { terminated } => {
                if !terminated {
                    err = "Missing trailing `'` symbol to terminate the character literal";
                }
                CHAR
            }
            lexer::LiteralKind::Byte { terminated } => {
                if !terminated {
                    err = "Missing trailing `'` symbol to terminate the byte literal";
                }
                BYTE
            }
            lexer::LiteralKind::Str { terminated } => {
                if !terminated {
                    err = "Missing trailing `\"` symbol to terminate the string literal";
                }
                STRING
            }
            lexer::LiteralKind::ByteStr { terminated } => {
                if !terminated {
                    err = "Missing trailing `\"` symbol to terminate the byte string literal";
                }
                BYTE_STRING
            }
            lexer::LiteralKind::RawStr { n_start_hashes, n_end_hashes, bad_char } => {
                if n_start_hashes > 255 {
                    err = "Too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols";
                }
                if bad_char.is_some() {
                    err = "Missing `\"` symbol after `#` symbols to begin the raw string literal";
                }
                if n_start_hashes != n_end_hashes {
                    err =
                        "Missing trailing `\"` with `#` symbols to terminate the raw string literal"
                }

                STRING
            }
            lexer::LiteralKind::RawByteStr { n_start_hashes, n_end_hashes, bad_char } => {
                if n_start_hashes > 255 {
                    err = "Too many `#` symbols: raw byte strings may be delimited by up to 255 `#` symbols";
                }
                if bad_char.is_some() {
                    err = "Missing `\"` symbol after `#` symbols to begin the raw byte string literal";
                }
                if n_start_hashes != n_end_hashes {
                    err =
                        "Missing trailing `\"` with `#` symbols to terminate the raw byte string literal"
                }

                BYTE_STRING
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, len, err);
    }
}
