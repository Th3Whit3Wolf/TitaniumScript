use lexer::{Base, BlockCommentToken, DocStyle, Lexer, LiteralKind, TokenKind};
use rustc_lexer::tokenize as rustc_tokenize;
use similar_asserts::assert_eq;
use test_utils::expect::expect_eq;
#[cfg(test)]
mod byte;
#[cfg(test)]
mod byte_str;
#[cfg(test)]
mod char;
#[cfg(test)]
mod comments;
#[cfg(test)]
mod float;
#[cfg(test)]
mod num;

#[cfg(test)]
mod str;

fn convert_doc_style(doc_style: Option<rustc_lexer::DocStyle>) -> Option<DocStyle> {
    match doc_style {
        Some(style) => match style {
            rustc_lexer::DocStyle::Outer => Some(DocStyle::Outer),
            rustc_lexer::DocStyle::Inner => Some(DocStyle::Inner),
        },
        None => None,
    }
}

fn convert_base(base: rustc_lexer::Base) -> Base {
    match base {
        rustc_lexer::Base::Binary => Base::Binary,
        rustc_lexer::Base::Octal => Base::Octal,
        rustc_lexer::Base::Hexadecimal => Base::Hexadecimal,
        rustc_lexer::Base::Decimal => Base::Decimal,
    }
}

pub(crate) fn convert_token(token: rustc_lexer::TokenKind) -> TokenKind {
    match token {
        rustc_lexer::TokenKind::LineComment { doc_style } => {
            TokenKind::LineComment(convert_doc_style(doc_style))
        }
        rustc_lexer::TokenKind::BlockComment { doc_style, terminated } => {
            TokenKind::BlockComment(BlockCommentToken {
                doc_style: convert_doc_style(doc_style),
                terminated,
            })
        }
        rustc_lexer::TokenKind::Whitespace => TokenKind::Whitespace,
        rustc_lexer::TokenKind::Ident => TokenKind::Ident,
        rustc_lexer::TokenKind::RawIdent => TokenKind::RawIdent,
        rustc_lexer::TokenKind::UnknownPrefix => TokenKind::Unknown,
        rustc_lexer::TokenKind::Literal { kind, suffix_start: _ } => match kind {
            rustc_lexer::LiteralKind::Int { base, empty_int } => {
                TokenKind::Literal(LiteralKind::Num { base: convert_base(base), empty_int })
            }
            rustc_lexer::LiteralKind::Float { base, empty_exponent } => {
                TokenKind::Literal(LiteralKind::Float { base: convert_base(base), empty_exponent })
            }
            rustc_lexer::LiteralKind::Char { terminated } => {
                TokenKind::Literal(LiteralKind::Char { terminated })
            }
            rustc_lexer::LiteralKind::Byte { terminated } => {
                TokenKind::Literal(LiteralKind::Byte { terminated })
            }
            rustc_lexer::LiteralKind::Str { terminated } => {
                TokenKind::Literal(LiteralKind::Str { terminated })
            }
            rustc_lexer::LiteralKind::ByteStr { terminated } => {
                TokenKind::Literal(LiteralKind::ByteStr { terminated })
            }
            rustc_lexer::LiteralKind::RawStr { n_hashes: _, err: _ } => unreachable!(),
            rustc_lexer::LiteralKind::RawByteStr { n_hashes: _, err: _ } => unreachable!(),
        },
        rustc_lexer::TokenKind::Lifetime { starts_with_number: _ } => unreachable!(),
        rustc_lexer::TokenKind::Semi => TokenKind::Semi,
        rustc_lexer::TokenKind::Comma => TokenKind::Comma,
        rustc_lexer::TokenKind::Dot => TokenKind::Dot,
        rustc_lexer::TokenKind::OpenParen => TokenKind::OpenParen,
        rustc_lexer::TokenKind::CloseParen => TokenKind::CloseParen,
        rustc_lexer::TokenKind::OpenBrace => TokenKind::OpenBrace,
        rustc_lexer::TokenKind::CloseBrace => TokenKind::CloseBrace,
        rustc_lexer::TokenKind::OpenBracket => TokenKind::OpenBracket,
        rustc_lexer::TokenKind::CloseBracket => TokenKind::CloseBracket,
        rustc_lexer::TokenKind::At => TokenKind::At,
        rustc_lexer::TokenKind::Pound => TokenKind::Pound,
        rustc_lexer::TokenKind::Tilde => TokenKind::Tilde,
        rustc_lexer::TokenKind::Question => TokenKind::Question,
        rustc_lexer::TokenKind::Colon => TokenKind::Colon,
        rustc_lexer::TokenKind::Dollar => TokenKind::Dollar,
        rustc_lexer::TokenKind::Eq => TokenKind::Eq,
        rustc_lexer::TokenKind::Bang => TokenKind::Bang,
        rustc_lexer::TokenKind::Lt => TokenKind::Lt,
        rustc_lexer::TokenKind::Gt => TokenKind::Gt,
        rustc_lexer::TokenKind::Minus => TokenKind::Minus,
        rustc_lexer::TokenKind::And => TokenKind::And,
        rustc_lexer::TokenKind::Or => TokenKind::Or,
        rustc_lexer::TokenKind::Plus => TokenKind::Plus,
        rustc_lexer::TokenKind::Star => TokenKind::Star,
        rustc_lexer::TokenKind::Slash => TokenKind::Slash,
        rustc_lexer::TokenKind::Caret => TokenKind::Caret,
        rustc_lexer::TokenKind::Percent => TokenKind::Percent,
        rustc_lexer::TokenKind::Unknown => TokenKind::Unknown,
    }
}

pub(crate) fn check(input: &str) {
    let mut lexer = Lexer::new(input);
    let token = lexer.next().unwrap();
    let mut rustc_lexer = rustc_tokenize(input);
    let rustc_token_raw = rustc_lexer.next().unwrap();
    let rustc_token = convert_token(rustc_token_raw.kind);

    // dbg!("Input: {}\nRust: {:?}\nTI: {:?}", input, &rustc_token_raw, &token);

    println!("INPUT: `{input}`");
    assert_eq!(token.kind, rustc_token);
    assert_eq!(token.range.end(), text_size::TextSize::from(rustc_token_raw.len as u32));
}

pub(crate) fn check_lexing(src: &str, expect: &str) {
    let actual: String = Lexer::new(src).map(|token| format!("{token:?}\n")).collect();
    expect_eq(expect, &actual)
}
