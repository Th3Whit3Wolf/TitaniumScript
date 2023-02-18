use crate::{flags, project_root};
mod extract;
mod generate;
mod lower;
mod utils;
mod ast;
use crate::grammar::{
    ast::{AstEnumSrc, AstNodeSrc, AstSrc, Cardinality, KINDS_SRC},
    extract::{extract_enum_traits, extract_enums, extract_struct_traits},
    generate::{generate_nodes, generate_syntax_kinds, generate_tokens},
    lower::{lower_enum, lower_rule},
    utils::{deduplicate_fields, ensure_file_contents, titaniumscript_grammar},
};
use quote::{format_ident, quote};
use ungrammar::Grammar;
use xshell::Shell;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

impl Field {
    fn is_many(&self) -> bool {
        matches!(self, Field::Node { cardinality: Cardinality::Many, .. })
    }
    fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = token.parse().unwrap();
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }
    fn method_name(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(name) => {
                let name = match name.as_str() {
                    ";" => "semicolon",
                    "->" => "thin_arrow",
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    "<" => "l_angle",
                    ">" => "r_angle",
                    "=" => "eq",
                    "!" => "excl",
                    "*" => "star",
                    "&" => "amp",
                    "_" => "underscore",
                    "." => "dot",
                    ".." => "dotdot",
                    "..." => "dotdotdot",
                    "..=" => "dotdoteq",
                    "=>" => "fat_arrow",
                    "@" => "at",
                    ":" => "colon",
                    "::" => "coloncolon",
                    "#" => "pound",
                    "?" => "question_mark",
                    "," => "comma",
                    "|" => "pipe",
                    "~" => "tilde",
                    _ => name,
                };
                format_ident!("{}_token", name)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", name)
                }
                // format_ident!("{}", name)
            }
        }
    }
    fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

pub(crate) fn lower(grammar: &Grammar) -> AstSrc {
    let mut res = AstSrc {
        tokens: "Whitespace Comment String ByteString IntNumber FloatNumber Char Byte Ident"
            .split_ascii_whitespace()
            .map(|it| it.to_string())
            .collect::<Vec<_>>(),
        ..Default::default()
    };

    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;
        match lower_enum(grammar, rule) {
            Some(variants) => {
                let enum_src = AstEnumSrc { doc: Vec::new(), name, traits: Vec::new(), variants };
                res.enums.push(enum_src);
            }
            None => {
                let mut fields = Vec::new();
                lower_rule(&mut fields, grammar, None, rule);
                res.nodes.push(AstNodeSrc { doc: Vec::new(), name, traits: Vec::new(), fields });
            }
        }
    }

    deduplicate_fields(&mut res);
    extract_enums(&mut res);
    extract_struct_traits(&mut res);
    extract_enum_traits(&mut res);
    res
}

impl AstNodeSrc {
    fn remove_field(&mut self, to_remove: Vec<usize>) {
        to_remove.into_iter().rev().for_each(|idx| {
            self.fields.remove(idx);
        });
    }
}

impl flags::Grammar {
    pub(crate) fn run(self, _sh: &Shell) -> anyhow::Result<()> {
        let syntax_kinds = generate_syntax_kinds(KINDS_SRC);
        let syntax_kinds_file = project_root().join("crates/parser/src/syntax_kind/generated.rs");
        ensure_file_contents(syntax_kinds_file.as_path(), &syntax_kinds);

        let grammar = titaniumscript_grammar();
        let ast = lower(&grammar);
        let ast_tokens = generate_tokens(&ast);
        let ast_tokens_file = project_root().join("crates/syntax/src/ast/generated/tokens.rs");
        ensure_file_contents(ast_tokens_file.as_path(), &ast_tokens);

        let ast_nodes = generate_nodes(KINDS_SRC, &ast);
        let ast_nodes_file = project_root().join("crates/syntax/src/ast/generated/nodes.rs");
        ensure_file_contents(ast_nodes_file.as_path(), &ast_nodes);
        Ok(())
    }
}
