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
mod raw_byte_str;
#[cfg(test)]
mod raw_str;
#[cfg(test)]
mod str;

use crate::{
    token_kind::{Base, LiteralKind},
    Lexer, TokenKind,
};

use expect_test::Expect;

pub fn check(input: &str, expected: TokenKind) {
    let mut lexer = Lexer::new(input);
    let token = lexer.next().unwrap();
    assert_eq!(token.kind, expected);
    assert_eq!(token.text, input);
}

pub fn check_lexing(src: &str, expect: Expect) {
    let actual: String = Lexer::new(src).map(|token| format!("{token:?}\n")).collect();
    expect.assert_eq(&actual)
}
