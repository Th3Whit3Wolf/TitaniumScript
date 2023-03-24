use super::check_parser_parser as check;

#[cfg(test)]
mod err {
    use super::*;

    #[test]
    fn test_0000_struct_field_missing_comma() {
        check("parser/err/0000_struct_field_missing_comma");
    }
    #[test]
    fn test_0001_item_recovery_in_file() {
        check("parser/err/0001_item_recovery_in_file");
    }
    #[test]
    fn test_0003_cpp_semicolon() {
        check("parser/err/0003_C++_semicolon");
    }
    #[test]
    fn test_0004_use_path_bad_segment() {
        check("parser/err/0004_use_path_bad_segment");
    }
    #[test]
    fn test_0005_attribute_recover() {
        check("parser/err/0005_attribute_recover");
    }
    #[test]
    fn test_0006_named_field_recovery() {
        check("parser/err/0006_named_field_recovery");
    }
    #[test]
    fn test_0007_stray_curly_in_file() {
        check("parser/err/0007_stray_curly_in_file");
    }
    #[test]
    fn test_0008_item_block_recovery() {
        check("parser/err/0008_item_block_recovery");
    }
    #[test]
    fn test_0009_broken_struct_type_parameter() {
        check("parser/err/0009_broken_struct_type_parameter");
    }
    #[test]
    fn test_0010_unsafe_lambda_block() {
        check("parser/err/0010_unsafe_lambda_block");
    }
    #[test]
    fn test_0011_extern_struct() {
        check("parser/err/0011_extern_struct");
    }
    #[test]
    fn test_0013_invalid_type() {
        check("parser/err/0013_invalid_type");
    }
    #[test]
    fn test_0014_where_no_bounds() {
        check("parser/err/0014_where_no_bounds");
    }
    #[test]
    fn test_0015_curly_in_params() {
        check("parser/err/0015_curly_in_params");
    }
    #[test]
    fn test_0016_missing_semi() {
        check("parser/err/0016_missing_semi");
    }
    #[test]
    fn test_0017_incomplete_binexpr() {
        check("parser/err/0017_incomplete_binexpr");
    }
    #[test]
    fn test_0018_incomplete_fn() {
        check("parser/err/0018_incomplete_fn");
    }
    #[test]
    fn test_0019_let_recover() {
        check("parser/err/0019_let_recover");
    }
    #[test]
    fn test_0020_fn_recover() {
        check("parser/err/0020_fn_recover");
    }
    #[test]
    fn test_0021_incomplete_param() {
        check("parser/err/0021_incomplete_param");
    }
    #[test]
    fn test_0022_bad_exprs() {
        check("parser/err/0022_bad_exprs");
    }
    #[test]
    fn test_0023_mismatched_paren() {
        check("parser/err/0023_mismatched_paren");
    }
    // #[test]
    // fn test_0024_many_type_parens() {
    //     check("parser/err/0024_many_type_parens");
    // }
    #[test]
    fn test_0025_nope() {
        check("parser/err/0025_nope");
    }
    #[test]
    fn test_0026_imp_recovery() {
        check("parser/err/0026_imp_recovery");
    }
    // #[test]
    // fn test_0027_incomplere_where_for() {
    //     check("parser/err/0027_incomplere_where_for");
    // }
    #[test]
    fn test_0029_field_completion() {
        check("parser/err/0029_field_completion");
    }
    #[test]
    fn test_0032_match_arms_inner_attrs() {
        check("parser/err/0032_match_arms_inner_attrs");
    }
    #[test]
    fn test_0033_match_arms_outer_attrs() {
        check("parser/err/0033_match_arms_outer_attrs");
    }
    #[test]
    fn test_0034_bad_box_pattern() {
        check("parser/err/0034_bad_box_pattern");
    }
    #[test]
    fn test_0035_use_recover() {
        check("parser/err/0035_use_recover");
    }
    #[test]
    fn test_0036_partial_use() {
        check("parser/err/0036_partial_use");
    }
    #[test]
    fn test_0039_lambda_recovery() {
        check("parser/err/0039_lambda_recovery");
    }
    // #[test]
    // fn test_0042_weird_blocks() {
    //     check("parser/err/0042_weird_blocks");
    // }
    // #[test]
    // fn test_0043_unexpected_for_type() {
    //     check("parser/err/0043_unexpected_for_type");
    // }
    #[test]
    fn test_0044_item_modifiers() {
        check("parser/err/0044_item_modifiers");
    }
    #[test]
    fn test_0047_repated_extern_modifier() {
        check("parser/err/0047_repated_extern_modifier");
    }
    #[test]
    fn test_0048_double_fish() {
        check("parser/err/0048_double_fish");
    }
}

#[cfg(test)]
mod ok {
    use super::*;

    #[test]
    fn test_0000_empty() {
        check("parser/ok/0000_empty");
    }
    #[test]
    fn test_0001_struct_item() {
        check("parser/ok/0001_struct_item");
    }
    #[test]
    fn test_0002_struct_item_field() {
        check("parser/ok/0002_struct_item_field");
    }
    #[test]
    fn test_0005_fn_item() {
        check("parser/ok/0005_fn_item");
    }
    #[test]
    fn test_0006_inner_attributes() {
        check("parser/ok/0006_inner_attributes");
    }
    #[test]
    fn test_0007_extern_crate() {
        check("parser/ok/0007_extern_crate");
    }
    #[test]
    fn test_0008_mod_item() {
        check("parser/ok/0008_mod_item");
    }
    #[test]
    fn test_0009_use_item() {
        check("parser/ok/0009_use_item");
    }
    #[test]
    fn test_0010_use_path_segments() {
        check("parser/ok/0010_use_path_segments");
    }
    #[test]
    fn test_0011_outer_attribute() {
        check("parser/ok/0011_outer_attribute");
    }
    #[test]
    fn test_0012_visibility() {
        check("parser/ok/0012_visibility");
    }
    #[test]
    fn test_0013_use_path_self_super() {
        check("parser/ok/0013_use_path_self_super");
    }
    #[test]
    fn test_0014_use_tree() {
        check("parser/ok/0014_use_tree");
    }
    #[test]
    fn test_0015_use_tree() {
        check("parser/ok/0015_use_tree");
    }
    #[test]
    fn test_0016_struct_flavors() {
        check("parser/ok/0016_struct_flavors");
    }
    #[test]
    fn test_0017_attr_trailing_comma() {
        check("parser/ok/0017_attr_trailing_comma");
    }
    #[test]
    fn test_0018_struct_type_params() {
        check("parser/ok/0018_struct_type_params");
    }
    #[test]
    fn test_0019_enums() {
        check("parser/ok/0019_enums");
    }
    #[test]
    fn test_0020_type_param_bounds() {
        check("parser/ok/0020_type_param_bounds");
    }
    #[test]
    fn test_0022_empty_extern_block() {
        check("parser/ok/0022_empty_extern_block");
    }
    #[test]
    fn test_0023_static_items() {
        check("parser/ok/0023_static_items");
    }
    #[test]
    fn test_0024_const_item() {
        check("parser/ok/0024_const_item");
    }
    #[test]
    fn test_0025_extern_fn_in_block() {
        check("parser/ok/0025_extern_fn_in_block");
    }
    #[test]
    fn test_0026_const_fn_in_block() {
        check("parser/ok/0026_const_fn_in_block");
    }
    #[test]
    fn test_0027_unsafe_fn_in_block() {
        check("parser/ok/0027_unsafe_fn_in_block");
    }
    #[test]
    fn test_0028_operator_binding_power() {
        check("parser/ok/0028_operator_binding_power");
    }
    #[test]
    fn test_0029_range_forms() {
        check("parser/ok/0029_range_forms");
    }
    #[test]
    fn test_0030_string_suffixes() {
        check("parser/ok/0030_string_suffixes");
    }
    #[test]
    fn test_0030_traits() {
        check("parser/ok/0030_traits");
    }
    #[test]
    fn test_0031_extern() {
        check("parser/ok/0031_extern");
    }
    #[test]
    fn test_0032_where_for() {
        check("parser/ok/0032_where_for");
    }
    #[test]
    fn test_0033_label_break() {
        check("parser/ok/0033_label_break");
    }
    #[test]
    fn test_0034_crate_path_in_call() {
        check("parser/ok/0034_crate_path_in_call");
    }
    #[test]
    fn test_0035_weird_exprs() {
        check("parser/ok/0035_weird_exprs");
    }
    #[test]
    fn test_0036_fully_qualified() {
        check("parser/ok/0036_fully_qualified");
    }
    #[test]
    fn test_0037_mod() {
        check("parser/ok/0037_mod");
    }
    #[test]
    fn test_0038_where_pred_type() {
        check("parser/ok/0038_where_pred_type");
    }
    #[test]
    fn test_0039_raw_fn_item() {
        check("parser/ok/0039_raw_fn_item");
    }
    #[test]
    fn test_0040_raw_struct_item_field() {
        check("parser/ok/0040_raw_struct_item_field");
    }
    #[test]
    fn test_0041_raw_keywords() {
        check("parser/ok/0041_raw_keywords");
    }
    #[test]
    fn test_0042_ufcs_call_list() {
        check("parser/ok/0042_ufcs_call_list");
    }
    #[test]
    fn test_0043_complex_assignment() {
        check("parser/ok/0043_complex_assignment");
    }
    #[test]
    fn test_0044_let_attrs() {
        check("parser/ok/0044_let_attrs");
    }
    #[test]
    fn test_0045_block_attrs() {
        check("parser/ok/0045_block_attrs");
    }
    #[test]
    fn test_0046_extern_inner_attributes() {
        check("parser/ok/0046_extern_inner_attributes");
    }
    #[test]
    fn test_0047_minus_in_inner_pattern() {
        check("parser/ok/0047_minus_in_inner_pattern");
    }
    #[test]
    fn test_0048_compound_assignment() {
        check("parser/ok/0048_compound_assignment");
    }
    #[test]
    fn test_0049_async_block() {
        check("parser/ok/0049_async_block");
    }
    #[test]
    fn test_0050_async_block_as_argument() {
        check("parser/ok/0050_async_block_as_argument");
    }
    #[test]
    fn test_0051_parameter_attrs() {
        check("parser/ok/0051_parameter_attrs");
    }
    #[test]
    fn test_0052_for_range_block() {
        check("parser/ok/0052_for_range_block");
    }
    #[test]
    fn test_0053_outer_attribute_on_macro_rules() {
        check("parser/ok/0053_outer_attribute_on_macro_rules");
    }
    #[test]
    fn test_0054_qual_path_in_type_arg() {
        check("parser/ok/0054_qual_path_in_type_arg");
    }
    #[test]
    fn test_0055_dot_dot_dot() {
        check("parser/ok/0055_dot_dot_dot");
    }
    #[test]
    fn test_0056_neq_in_type() {
        check("parser/ok/0056_neq_in_type");
    }
    #[test]
    fn test_0057_loop_in_call() {
        check("parser/ok/0057_loop_in_call");
    }
    #[test]
    fn test_0058_unary_expr_precedence() {
        check("parser/ok/0058_unary_expr_precedence");
    }
    #[test]
    fn test_0059_loops_in_parens() {
        check("parser/ok/0059_loops_in_parens");
    }
    #[test]
    fn test_0060_as_range() {
        check("parser/ok/0060_as_range");
    }
    #[test]
    fn test_0061_match_full_range() {
        check("parser/ok/0061_match_full_range");
    }
    #[test]
    fn test_0062_macro_2_0() {
        check("parser/ok/0062_macro_2.0");
    }
    #[test]
    fn test_0063_trait_fn_patterns() {
        check("parser/ok/0063_trait_fn_patterns");
    }
    #[test]
    fn test_0063_variadic_fun() {
        check("parser/ok/0063_variadic_fun");
    }
    #[test]
    fn test_0064_impl_fn_params() {
        check("parser/ok/0064_impl_fn_params");
    }
    #[test]
    fn test_0065_comment_newline() {
        check("parser/ok/0065_comment_newline");
    }
    #[test]
    fn test_0065_plus_after_fn_trait_bound() {
        check("parser/ok/0065_plus_after_fn_trait_bound");
    }
    #[test]
    fn test_0066_default_modifier() {
        check("parser/ok/0066_default_modifier");
    }
    #[test]
    fn test_0067_where_for_pred() {
        check("parser/ok/0067_where_for_pred");
    }
    #[test]
    fn test_0068_item_modifiers() {
        check("parser/ok/0068_item_modifiers");
    }
    #[test]
    fn test_0069_multi_trait_object() {
        check("parser/ok/0069_multi_trait_object");
    }
    #[test]
    fn test_0070_expr_attr_placement() {
        check("parser/ok/0070_expr_attr_placement");
    }
    #[test]
    fn test_0071_stmt_attr_placement() {
        check("parser/ok/0071_stmt_attr_placement");
    }
    #[test]
    fn test_0072_destructuring_assignment() {
        check("parser/ok/0072_destructuring_assignment");
    }
}
