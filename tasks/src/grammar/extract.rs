use crate::grammar::{
    ast::{AstNodeSrc, AstSrc, Cardinality},
    utils::to_lower_snake_case,
    Field,
};
use std::collections::BTreeSet;

pub(crate) fn extract_enums(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        for enm in &ast.enums {
            let mut to_remove = Vec::new();
            for (i, field) in node.fields.iter().enumerate() {
                let ty = field.ty().to_string();
                if enm.variants.iter().any(|it| it == &ty) {
                    to_remove.push(i);
                }
            }
            if to_remove.len() == enm.variants.len() {
                node.remove_field(to_remove);
                let ty = enm.name.clone();
                let name = to_lower_snake_case(&ty);
                node.fields.push(Field::Node { name, ty, cardinality: Cardinality::Optional });
            }
        }
    }
}

pub(crate) fn extract_struct_traits(ast: &mut AstSrc) {
    let traits: &[(&str, &[&str])] = &[
        ("HasAttrs", &["attrs"]),
        ("HasName", &["name"]),
        ("HasVisibility", &["visibility"]),
        ("HasGenericParams", &["generic_param_list", "where_clause"]),
        ("HasTypeBounds", &["type_bound_list", "colon_token"]),
        ("HasModuleItem", &["items"]),
        ("HasLoopBody", &["label", "loop_body"]),
        ("HasArgList", &["arg_list"]),
    ];

    for node in &mut ast.nodes {
        for (name, methods) in traits {
            extract_struct_trait(node, name, methods);
        }
    }

    let nodes_with_doc_comments = [
        "SourceFile",
        "Fn",
        "Struct",
        "Union",
        "RecordField",
        "TupleField",
        "Enum",
        "Variant",
        "Trait",
        "Module",
        "Static",
        "Const",
        "TypeAlias",
        "Impl",
        "ExternBlock",
        "ExternCrate",
        "MacroCall",
        "MacroRules",
        "MacroDef",
        "Use",
    ];

    for node in &mut ast.nodes {
        if nodes_with_doc_comments.contains(&&*node.name) {
            node.traits.push("HasDocComments".into());
        }
    }
}

pub(crate) fn extract_struct_trait(node: &mut AstNodeSrc, trait_name: &str, methods: &[&str]) {
    let mut to_remove = Vec::new();
    for (i, field) in node.fields.iter().enumerate() {
        let method_name = field.method_name().to_string();
        if methods.iter().any(|&it| it == method_name) {
            to_remove.push(i);
        }
    }
    if to_remove.len() == methods.len() {
        node.traits.push(trait_name.to_string());
        node.remove_field(to_remove);
    }
}

pub(crate) fn extract_enum_traits(ast: &mut AstSrc) {
    for enm in &mut ast.enums {
        if enm.name == "Stmt" {
            continue;
        }
        let nodes = &ast.nodes;
        let mut variant_traits = enm
            .variants
            .iter()
            .map(|var| nodes.iter().find(|it| &it.name == var).unwrap())
            .map(|node| node.traits.iter().cloned().collect::<BTreeSet<_>>());

        let mut enum_traits = match variant_traits.next() {
            Some(it) => it,
            None => continue,
        };
        for traits in variant_traits {
            enum_traits = enum_traits.intersection(&traits).cloned().collect();
        }
        enm.traits = enum_traits.into_iter().collect();
    }
}
