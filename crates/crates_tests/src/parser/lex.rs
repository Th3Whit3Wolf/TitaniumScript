use super::check_parser_lex as check;

#[cfg(test)]
mod err {
    use super::*;
    #[test]
    fn empty_exponent() {
        check("lexer/err/empty_exponent");
    }
    #[test]
    fn empty_int() {
        check("lexer/err/empty_int");
    }
    #[test]
    fn unclosed_block_comment_at_eof() {
        check("lexer/err/unclosed_block_comment_at_eof");
    }
    #[test]
    fn unclosed_block_comment_with_content() {
        check("lexer/err/unclosed_block_comment_with_content");
    }
    #[test]
    fn unclosed_byte_at_eof() {
        check("lexer/err/unclosed_byte_at_eof");
    }
    #[test]
    fn unclosed_byte_string_at_eof() {
        check("lexer/err/unclosed_byte_string_at_eof");
    }
    #[test]
    fn unclosed_byte_string_with_ascii_escape() {
        check("lexer/err/unclosed_byte_string_with_ascii_escape");
    }
    #[test]
    fn unclosed_byte_string_with_ferris() {
        check("lexer/err/unclosed_byte_string_with_ferris");
    }
    #[test]
    fn unclosed_byte_string_with_slash() {
        check("lexer/err/unclosed_byte_string_with_slash");
    }
    #[test]
    fn unclosed_byte_string_with_slash_double_quote() {
        check("lexer/err/unclosed_byte_string_with_slash_double_quote");
    }
    #[test]
    fn unclosed_byte_string_with_slash_n() {
        check("lexer/err/unclosed_byte_string_with_slash_n");
    }
    #[test]
    fn unclosed_byte_string_with_space() {
        check("lexer/err/unclosed_byte_string_with_space");
    }
    #[test]
    fn unclosed_byte_string_with_unicode_escape() {
        check("lexer/err/unclosed_byte_string_with_unicode_escape");
    }
    #[test]
    fn unclosed_byte_with_ascii_escape() {
        check("lexer/err/unclosed_byte_with_ascii_escape");
    }
    #[test]
    fn unclosed_byte_with_ferris() {
        check("lexer/err/unclosed_byte_with_ferris");
    }
    #[test]
    fn unclosed_byte_with_slash() {
        check("lexer/err/unclosed_byte_with_slash");
    }
    #[test]
    fn unclosed_byte_with_slash_n() {
        check("lexer/err/unclosed_byte_with_slash_n");
    }
    #[test]
    fn unclosed_byte_with_slash_single_quote() {
        check("lexer/err/unclosed_byte_with_slash_single_quote");
    }
    #[test]
    fn unclosed_byte_with_space() {
        check("lexer/err/unclosed_byte_with_space");
    }
    #[test]
    fn unclosed_byte_with_unicode_escape() {
        check("lexer/err/unclosed_byte_with_unicode_escape");
    }
    #[test]
    fn unclosed_char_at_eof() {
        check("lexer/err/unclosed_char_at_eof");
    }
    #[test]
    fn unclosed_char_with_ascii_escape() {
        check("lexer/err/unclosed_char_with_ascii_escape");
    }
    #[test]
    fn unclosed_char_with_ferris() {
        check("lexer/err/unclosed_char_with_ferris");
    }
    #[test]
    fn unclosed_char_with_slash() {
        check("lexer/err/unclosed_char_with_slash");
    }
    #[test]
    fn unclosed_char_with_slash_n() {
        check("lexer/err/unclosed_char_with_slash_n");
    }
    #[test]
    fn unclosed_char_with_slash_single_quote() {
        check("lexer/err/unclosed_char_with_slash_single_quote");
    }
    #[test]
    fn unclosed_char_with_space() {
        check("lexer/err/unclosed_char_with_space");
    }
    #[test]
    fn unclosed_char_with_unicode_escape() {
        check("lexer/err/unclosed_char_with_unicode_escape");
    }
    #[test]
    fn unclosed_nested_block_comment_entirely() {
        check("lexer/err/unclosed_nested_block_comment_entirely");
    }
    #[test]
    fn unclosed_nested_block_comment_partially() {
        check("lexer/err/unclosed_nested_block_comment_partially");
    }
    #[test]
    fn unclosed_string_at_eof() {
        check("lexer/err/unclosed_string_at_eof");
    }
    #[test]
    fn unclosed_string_with_ascii_escape() {
        check("lexer/err/unclosed_string_with_ascii_escape");
    }
    #[test]
    fn unclosed_string_with_ferris() {
        check("lexer/err/unclosed_string_with_ferris");
    }
    #[test]
    fn unclosed_string_with_slash() {
        check("lexer/err/unclosed_string_with_slash");
    }
    #[test]
    fn unclosed_string_with_slash_double_quote() {
        check("lexer/err/unclosed_string_with_slash_double_quote");
    }
    #[test]
    fn unclosed_string_with_slash_n() {
        check("lexer/err/unclosed_string_with_slash_n");
    }
    #[test]
    fn unclosed_string_with_space() {
        check("lexer/err/unclosed_string_with_space");
    }
    #[test]
    fn unclosed_string_with_unicode_escape() {
        check("lexer/err/unclosed_string_with_unicode_escape");
    }
}

#[cfg(test)]
mod ok {
    use super::*;

    #[test]
    fn block_comment() {
        check("lexer/ok/block_comment");
    }
    #[test]
    fn byte_strings() {
        check("lexer/ok/byte_strings");
    }
    #[test]
    fn chars() {
        check("lexer/ok/chars");
    }
    #[test]
    fn hello() {
        check("lexer/ok/hello");
    }
    #[test]
    fn ident() {
        check("lexer/ok/ident");
    }
    #[test]
    fn keywords() {
        check("lexer/ok/keywords");
    }
    #[test]
    fn numbers() {
        check("lexer/ok/numbers");
    }
    #[test]
    fn raw_ident() {
        check("lexer/ok/raw_ident");
    }

    #[test]
    fn single_line_comments() {
        check("lexer/ok/single_line_comments");
    }
    #[test]
    fn strings() {
        check("lexer/ok/strings");
    }
    #[test]
    fn symbols() {
        check("lexer/ok/symbols");
    }
    #[test]
    fn whitespace() {
        check("lexer/ok/whitespace");
    }
}
