// use super::check_parser_parser as check;

// #[cfg(test)]
// mod err {
//     use super::*;

//     #[test]
//     fn test_0001_array_type_missing_semi() {
//         check("parser/inline/err/0001_array_type_missing_semi");
//     }
//     #[test]
//     fn test_0002_misplaced_label_err() {
//         check("parser/inline/err/0002_misplaced_label_err");
//     }
//     #[test]
//     fn test_0003_pointer_type_no_mutability() {
//         check("parser/inline/err/0003_pointer_type_no_mutability");
//     }
//     #[test]
//     fn test_0004_impl_type() {
//         check("parser/inline/err/0004_impl_type");
//     }
//     #[test]
//     fn test_0005_fn_pointer_type_missing_fn() {
//         check("parser/inline/err/0005_fn_pointer_type_missing_fn");
//     }
//     #[test]
//     fn test_0006_unsafe_block_in_mod() {
//         check("parser/inline/err/0006_unsafe_block_in_mod");
//     }
//     #[test]
//     fn test_0007_async_without_semicolon() {
//         check("parser/inline/err/0007_async_without_semicolon");
//     }
//     #[test]
//     fn test_0008_pub_expr() {
//         check("parser/inline/err/0008_pub_expr");
//     }
//     #[test]
//     fn test_0013_anonymous_static() {
//         check("parser/inline/err/0013_anonymous_static");
//     }
//     #[test]
//     fn test_0014_record_literal_before_ellipsis_recovery() {
//         check("parser/inline/err/0014_record_literal_before_ellipsis_recovery");
//     }
//     #[test]
//     fn test_0014_record_literal_missing_ellipsis_recovery() {
//         check("parser/inline/err/0014_record_literal_missing_ellipsis_recovery");
//     }
//     #[test]
//     fn test_0014_struct_field_recover() {
//         check("parser/inline/err/0014_struct_field_recover");
//     }
//     #[test]
//     fn test_0015_empty_segment() {
//         check("parser/inline/err/0015_empty_segment");
//     }
//     #[test]
//     fn test_0015_missing_fn_param_type() {
//         check("parser/inline/err/0015_missing_fn_param_type");
//     }
// }

// #[cfg(test)]
// mod ok {
//     use super::*;

//     #[test]
//     fn test_0002_use_tree_list() {
//         check("parser/inline/ok/0002_use_tree_list");
//     }
//     #[test]
//     fn test_0003_where_pred_for() {
//         check("parser/inline/ok/0003_where_pred_for");
//     }
//     #[test]
//     fn test_0004_value_parameters_no_patterns() {
//         check("parser/inline/ok/0004_value_parameters_no_patterns");
//     }
//     #[test]
//     fn test_0005_function_type_params() {
//         check("parser/inline/ok/0005_function_type_params");
//     }
//     #[test]
//     fn test_0006_self_param() {
//         check("parser/inline/ok/0006_self_param");
//     }
//     #[test]
//     fn test_0007_type_param_bounds() {
//         check("parser/inline/ok/0007_type_param_bounds");
//     }
//     #[test]
//     fn test_0008_path_part() {
//         check("parser/inline/ok/0008_path_part");
//     }
//     #[test]
//     fn test_0009_loop_expr() {
//         check("parser/inline/ok/0009_loop_expr");
//     }
//     #[test]
//     fn test_0010_extern_block() {
//         check("parser/inline/ok/0010_extern_block");
//     }
//     #[test]
//     fn test_0011_field_expr() {
//         check("parser/inline/ok/0011_field_expr");
//     }
//     #[test]
//     fn test_0012_type_item_where_clause() {
//         check("parser/inline/ok/0012_type_item_where_clause");
//     }
//     #[test]
//     fn test_0013_pointer_type_mut() {
//         check("parser/inline/ok/0013_pointer_type_mut");
//     }
//     #[test]
//     fn test_0014_never_type() {
//         check("parser/inline/ok/0014_never_type");
//     }
//     #[test]
//     fn test_0015_continue_expr() {
//         check("parser/inline/ok/0015_continue_expr");
//     }
//     #[test]
//     fn test_0017_array_type() {
//         check("parser/inline/ok/0017_array_type");
//     }
//     #[test]
//     fn test_0018_arb_self_types() {
//         check("parser/inline/ok/0018_arb_self_types");
//     }
//     #[test]
//     fn test_0019_unary_expr() {
//         check("parser/inline/ok/0019_unary_expr");
//     }
//     #[test]
//     fn test_0021_assoc_item_list() {
//         check("parser/inline/ok/0021_assoc_item_list");
//     }
//     #[test]
//     fn test_0022_crate_visibility() {
//         check("parser/inline/ok/0022_crate_visibility");
//     }
//     #[test]
//     fn test_0023_placeholder_type() {
//         check("parser/inline/ok/0023_placeholder_type");
//     }
//     #[test]
//     fn test_0024_slice_pat() {
//         check("parser/inline/ok/0024_slice_pat");
//     }
//     #[test]
//     fn test_0025_slice_type() {
//         check("parser/inline/ok/0025_slice_type");
//     }
//     #[test]
//     fn test_0026_tuple_pat_fields() {
//         check("parser/inline/ok/0026_tuple_pat_fields");
//     }
//     #[test]
//     fn test_0027_ref_pat() {
//         check("parser/inline/ok/0027_ref_pat");
//     }
//     #[test]
//     fn test_0028_impl_trait_type() {
//         check("parser/inline/ok/0028_impl_trait_type");
//     }
//     #[test]
//     fn test_0029_cast_expr() {
//         check("parser/inline/ok/0029_cast_expr");
//     }
//     #[test]
//     fn test_0030_let_expr() {
//         check("parser/inline/ok/0030_let_expr");
//     }
//     #[test]
//     fn test_0031_while_expr() {
//         check("parser/inline/ok/0031_while_expr");
//     }
//     #[test]
//     fn test_0032_fn_pointer_type() {
//         check("parser/inline/ok/0032_fn_pointer_type");
//     }
//     #[test]
//     fn test_0033_reference_type() {
//         check("parser/inline/ok/0033_reference_type;");
//     }
//     #[test]
//     fn test_0034_break_expr() {
//         check("parser/inline/ok/0034_break_expr");
//     }
//     #[test]
//     fn test_0037_qual_paths() {
//         check("parser/inline/ok/0037_qual_paths");
//     }
//     #[test]
//     fn test_0038_full_range_expr() {
//         check("parser/inline/ok/0038_full_range_expr");
//     }
//     #[test]
//     fn test_0040_crate_keyword_vis() {
//         check("parser/inline/ok/0040_crate_keyword_vis");
//     }
//     #[test]
//     fn test_0041_trait_item() {
//         check("parser/inline/ok/0041_trait_item");
//     }
//     #[test]
//     fn test_0042_call_expr() {
//         check("parser/inline/ok/0042_call_expr");
//     }
//     #[test]
//     fn test_0044_block_items() {
//         check("parser/inline/ok/0044_block_items");
//     }
//     #[test]
//     fn test_0045_param_list_opt_patterns() {
//         check("parser/inline/ok/0045_param_list_opt_patterns");
//     }
//     #[test]
//     fn test_0046_singleton_tuple_type() {
//         check("parser/inline/ok/0046_singleton_tuple_type");
//     }
//     #[test]
//     fn test_0048_path_type_with_bounds() {
//         check("parser/inline/ok/0048_path_type_with_bounds");
//     }
//     #[test]
//     fn test_0050_fn_decl() {
//         check("parser/inline/ok/0050_fn_decl");
//     }
//     #[test]
//     fn test_0051_unit_type() {
//         check("parser/inline/ok/0051_unit_type");
//     }
//     #[test]
//     fn test_0052_path_type() {
//         check("parser/inline/ok/0052_path_type");
//     }
//     #[test]
//     fn test_0053_path_expr() {
//         check("parser/inline/ok/0053_path_expr");
//     }
//     #[test]
//     fn test_0054_record_field_attrs() {
//         check("parser/inline/ok/0054_record_field_attrs");
//     }
//     #[test]
//     fn test_0055_literal_pattern() {
//         check("parser/inline/ok/0055_literal_pattern");
//     }
//     #[test]
//     fn test_0056_where_clause() {
//         check("parser/inline/ok/0056_where_clause");
//     }
//     #[test]
//     fn test_0058_range_pat() {
//         check("parser/inline/ok/0058_range_pat");
//     }
//     #[test]
//     fn test_0059_match_arms_commas() {
//         check("parser/inline/ok/0059_match_arms_commas");
//     }
//     #[test]
//     fn test_0060_extern_crate() {
//         check("parser/inline/ok/0060_extern_crate");
//     }
//     #[test]
//     fn test_0061_record_lit() {
//         check("parser/inline/ok/0061_record_lit");
//     }
//     #[test]
//     fn test_0062_mod_contents() {
//         check("parser/inline/ok/0062_mod_contents");
//     }
//     #[test]
//     fn test_0063_impl_item_neg() {
//         check("parser/inline/ok/0063_impl_item_neg");
//     }
//     #[test]
//     fn test_0064_if_expr() {
//         check("parser/inline/ok/0064_if_expr");
//     }
//     #[test]
//     fn test_0065_dyn_trait_type() {
//         check("parser/inline/ok/0065_dyn_trait_type");
//     }
//     #[test]
//     fn test_0066_match_arm() {
//         check("parser/inline/ok/0066_match_arm");
//     }
//     #[test]
//     fn test_0067_crate_path() {
//         check("parser/inline/ok/0067_crate_path");
//     }
//     #[test]
//     fn test_0070_stmt_bin_expr_ambiguity() {
//         check("parser/inline/ok/0070_stmt_bin_expr_ambiguity");
//     }
//     #[test]
//     fn test_0071_match_expr() {
//         check("parser/inline/ok/0071_match_expr");
//     }
//     #[test]
//     fn test_0072_return_expr() {
//         check("parser/inline/ok/0072_return_expr");
//     }
//     #[test]
//     fn test_0073_type_item_type_params() {
//         check("parser/inline/ok/0073_type_item_type_params");
//     }
//     #[test]
//     fn test_0074_stmt_postfix_expr_ambiguity() {
//         check("parser/inline/ok/0074_stmt_postfix_expr_ambiguity");
//     }
//     #[test]
//     fn test_0075_block() {
//         check("parser/inline/ok/0075_block");
//     }
//     #[test]
//     fn test_0076_function_where_clause() {
//         check("parser/inline/ok/0076_function_where_clause");
//     }
//     #[test]
//     fn test_0077_try_expr() {
//         check("parser/inline/ok/0077_try_expr");
//     }
//     #[test]
//     fn test_0078_type_alias() {
//         check("parser/inline/ok/0078_type_alias");
//     }
//     #[test]
//     fn test_0079_impl_item() {
//         check("parser/inline/ok/0079_impl_item");
//     }
//     #[test]
//     fn test_0080_postfix_range() {
//         check("parser/inline/ok/0080_postfix_range");
//     }
//     #[test]
//     fn test_0081_for_type() {
//         check("parser/inline/ok/0081_for_type");
//     }
//     #[test]
//     fn test_0082_ref_expr() {
//         check("parser/inline/ok/0082_ref_expr");
//     }
//     #[test]
//     fn test_0084_paren_type() {
//         check("parser/inline/ok/0084_paren_type");
//     }
//     #[test]
//     fn test_0085_expr_literals() {
//         check("parser/inline/ok/0085_expr_literals");
//     }
//     #[test]
//     fn test_0086_function_ret_type() {
//         check("parser/inline/ok/0086_function_ret_type");
//     }
//     #[test]
//     fn test_0088_break_ambiguity() {
//         check("parser/inline/ok/0088_break_ambiguity");
//     }
//     #[test]
//     fn test_0090_type_param_default() {
//         check("parser/inline/ok/0090_type_param_default");
//     }
//     #[test]
//     fn test_0092_fn_pointer_type_with_ret() {
//         check("parser/inline/ok/0092_fn_pointer_type_with_ret");
//     }
//     #[test]
//     fn test_0093_index_expr() {
//         check("parser/inline/ok/0093_index_expr");
//     }
//     #[test]
//     fn test_0095_placeholder_pat() {
//         check("parser/inline/ok/0095_placeholder_pat");
//     }
//     #[test]
//     fn test_0096_no_semi_after_block() {
//         check("parser/inline/ok/0096_no_semi_after_block");
//     }
//     #[test]
//     fn test_0099_param_list() {
//         check("parser/inline/ok/0099_param_list");
//     }
//     #[test]
//     fn test_0100_for_expr() {
//         check("parser/inline/ok/0100_for_expr");
//     }
//     #[test]
//     fn test_0102_record_pat_field_list() {
//         check("parser/inline/ok/0102_record_pat_field_list");
//     }
//     #[test]
//     fn test_0103_array_expr() {
//         check("parser/inline/ok/0103_array_expr");
//     }
//     #[test]
//     fn test_0104_path_fn_trait_args() {
//         check("parser/inline/ok/0104_path_fn_trait_args");
//     }
//     #[test]
//     fn test_0106_lambda_expr() {
//         check("parser/inline/ok/0106_lambda_expr");
//     }
//     #[test]
//     fn test_0107_method_call_expr() {
//         check("parser/inline/ok/0107_method_call_expr");
//     }
//     #[test]
//     fn test_0108_tuple_expr() {
//         check("parser/inline/ok/0108_tuple_expr");
//     }
//     #[test]
//     fn test_0109_label() {
//         check("parser/inline/ok/0109_label");
//     }
//     #[test]
//     fn test_0111_tuple_pat() {
//         check("parser/inline/ok/0111_tuple_pat");
//     }
//     #[test]
//     fn test_0112_bind_pat() {
//         check("parser/inline/ok/0112_bind_pat");
//     }
//     #[test]
//     fn test_0113_nocontentexpr() {
//         check("parser/inline/ok/0113_nocontentexpr");
//     }
//     #[test]
//     fn test_0114_tuple_struct_where() {
//         check("parser/inline/ok/0114_tuple_struct_where");
//     }
//     #[test]
//     fn test_0115_tuple_field_attrs() {
//         check("parser/inline/ok/0115_tuple_field_attrs");
//     }
//     #[test]
//     fn test_0117_macro_call_type() {
//         check("parser/inline/ok/0117_macro_call_type");
//     }
//     #[test]
//     fn test_0118_match_guard() {
//         check("parser/inline/ok/0118_match_guard");
//     }
//     #[test]
//     fn test_0120_match_arms_inner_attribute() {
//         check("parser/inline/ok/0120_match_arms_inner_attribute");
//     }
//     #[test]
//     fn test_0121_match_arms_outer_attributes() {
//         check("parser/inline/ok/0121_match_arms_outer_attributes");
//     }
//     #[test]
//     fn test_0123_param_list_vararg() {
//         check("parser/inline/ok/0123_param_list_vararg");
//     }
//     #[test]
//     fn test_0125_crate_keyword_path() {
//         check("parser/inline/ok/0125_crate_keyword_path");
//     }
//     #[test]
//     fn test_0125_record_literal_field_with_attr() {
//         check("parser/inline/ok/0125_record_literal_field_with_attr");
//     }
//     #[test]
//     fn test_0126_attr_on_expr_stmt() {
//         check("parser/inline/ok/0126_attr_on_expr_stmt");
//     }
//     #[test]
//     fn test_0129_marco_pat() {
//         check("parser/inline/ok/0129_marco_pat");
//     }
//     #[test]
//     fn test_0130_let_stmt() {
//         check("parser/inline/ok/0130_let_stmt");
//     }
//     #[test]
//     fn test_0130_try_block_expr() {
//         check("parser/inline/ok/0130_try_block_expr");
//     }
//     #[test]
//     fn test_0131_existential_type() {
//         check("parser/inline/ok/0131_existential_type");
//     }
//     #[test]
//     fn test_0132_box_expr() {
//         check("parser/inline/ok/0132_box_expr");
//     }
//     #[test]
//     fn test_0134_nocontentexpr_after_item() {
//         check("parser/inline/ok/0134_nocontentexpr_after_item");
//     }
//     #[test]
//     fn test_0137_await_expr() {
//         check("parser/inline/ok/0137_await_expr");
//     }
//     #[test]
//     fn test_0138_associated_type_bounds() {
//         check("parser/inline/ok/0138_associated_type_bounds");
//     }
//     #[test]
//     fn test_0138_expression_after_block() {
//         check("parser/inline/ok/0138_expression_after_block");
//     }
//     #[test]
//     fn test_0138_self_param_outer_attr() {
//         check("parser/inline/ok/0138_self_param_outer_attr");
//     }
//     #[test]
//     fn test_0139_param_outer_arg() {
//         check("parser/inline/ok/0139_param_outer_arg");
//     }
//     #[test]
//     fn test_0142_for_range_from() {
//         check("parser/inline/ok/0142_for_range_from");
//     }
//     #[test]
//     fn test_0143_box_pat() {
//         check("parser/inline/ok/0143_box_pat");
//     }
//     #[test]
//     fn test_0144_dot_dot_pat() {
//         check("parser/inline/ok/0144_dot_dot_pat");
//     }
//     #[test]
//     fn test_0145_record_pat_field() {
//         check("parser/inline/ok/0145_record_pat_field");
//     }
//     #[test]
//     fn test_0146_as_precedence() {
//         check("parser/inline/ok/0146_as_precedence");
//     }
//     #[test]
//     fn test_0147_const_param() {
//         check("parser/inline/ok/0147_const_param");
//     }
//     #[test]
//     fn test_0147_macro_def() {
//         check("parser/inline/ok/0147_macro_def");
//     }
//     #[test]
//     fn test_0150_array_attrs() {
//         check("parser/inline/ok/0150_array_attrs");
//     }
//     #[test]
//     fn test_0150_impl_type_params() {
//         check("parser/inline/ok/0150_impl_type_params");
//     }
//     #[test]
//     fn test_0151_fn() {
//         check("parser/inline/ok/0151_fn");
//     }
//     #[test]
//     fn test_0151_trait_alias() {
//         check("parser/inline/ok/0151_trait_alias");
//     }
//     #[test]
//     fn test_0152_arg_with_attr() {
//         check("parser/inline/ok/0152_arg_with_attr");
//     }
//     #[test]
//     fn test_0153_pub_parens_typepath() {
//         check("parser/inline/ok/0153_pub_parens_typepath");
//     }
//     #[test]
//     fn test_0154_fn_pointer_param_ident_path() {
//         check("parser/inline/ok/0154_fn_pointer_param_ident_path");
//     }
//     #[test]
//     fn test_0154_no_dyn_trait_leading_for() {
//         check("parser/inline/ok/0154_no_dyn_trait_leading_for");
//     }
//     #[test]
//     fn test_0154_tuple_attrs() {
//         check("parser/inline/ok/0154_tuple_attrs");
//     }
//     #[test]
//     fn test_0155_closure_params() {
//         check("parser/inline/ok/0155_closure_params");
//     }
//     #[test]
//     fn test_0156_const_block_pat() {
//         check("parser/inline/ok/0156_const_block_pat");
//     }
//     #[test]
//     fn test_0156_fn_def_param() {
//         check("parser/inline/ok/0156_fn_def_param");
//     }
//     #[test]
//     fn test_0156_or_pattern() {
//         check("parser/inline/ok/0156_or_pattern");
//     }
//     #[test]
//     fn test_0157_fn_pointer_unnamed_arg() {
//         check("parser/inline/ok/0157_fn_pointer_unnamed_arg");
//     }
//     #[test]
//     fn test_0157_variant_discriminant() {
//         check("parser/inline/ok/0157_variant_discriminant");
//     }
//     #[test]
//     fn test_0158_binop_resets_statementness() {
//         check("parser/inline/ok/0158_binop_resets_statementness");
//     }
//     #[test]
//     fn test_0158_lambda_ret_block() {
//         check("parser/inline/ok/0158_lambda_ret_block");
//     }
//     #[test]
//     fn test_0158_macro_rules_non_brace() {
//         check("parser/inline/ok/0158_macro_rules_non_brace");
//     }
//     #[test]
//     fn test_0159_try_macro_fallback() {
//         check("parser/inline/ok/0159_try_macro_fallback");
//     }
//     #[test]
//     fn test_0159_yield_expr() {
//         check("parser/inline/ok/0159_yield_expr");
//     }
//     #[test]
//     fn test_0160_crate_visibility_in() {
//         check("parser/inline/ok/0160_crate_visibility_in");
//     }
//     #[test]
//     fn test_0160_try_macro_rules() {
//         check("parser/inline/ok/0160_try_macro_rules");
//     }
//     #[test]
//     fn test_0161_impl_item_const() {
//         check("parser/inline/ok/0161_impl_item_const");
//     }
//     #[test]
//     fn test_0161_labeled_block() {
//         check("parser/inline/ok/0161_labeled_block");
//     }
//     #[test]
//     fn test_0162_default_async_unsafe_fn() {
//         check("parser/inline/ok/0162_default_async_unsafe_fn");
//     }
//     #[test]
//     fn test_0163_default_async_fn() {
//         check("parser/inline/ok/0163_default_async_fn");
//     }
//     #[test]
//     fn test_0163_default_unsafe_item() {
//         check("parser/inline/ok/0163_default_unsafe_item");
//     }
//     #[test]
//     fn test_0164_default_item() {
//         check("parser/inline/ok/0164_default_item");
//     }
//     #[test]
//     fn test_0164_type_path_in_pattern() {
//         check("parser/inline/ok/0164_type_path_in_pattern");
//     }
//     #[test]
//     fn test_0166_half_open_range_pat() {
//         check("parser/inline/ok/0166_half_open_range_pat");
//     }
//     #[test]
//     fn test_0168_extern_crate_rename() {
//         check("parser/inline/ok/0168_extern_crate_rename");
//     }
//     #[test]
//     fn test_0168_extern_crate_self() {
//         check("parser/inline/ok/0168_extern_crate_self");
//     }
//     #[test]
//     fn test_0169_mod_item() {
//         check("parser/inline/ok/0169_mod_item");
//     }
//     #[test]
//     fn test_0170_mod_item_curly() {
//         check("parser/inline/ok/0170_mod_item_curly");
//     }
//     #[test]
//     fn test_0170_tuple_struct() {
//         check("parser/inline/ok/0170_tuple_struct");
//     }
//     #[test]
//     fn test_0171_struct_item() {
//         check("parser/inline/ok/0171_struct_item");
//     }
//     #[test]
//     fn test_0172_const_item() {
//         check("parser/inline/ok/0172_const_item");
//     }
//     #[test]
//     fn test_0172_record_field_list() {
//         check("parser/inline/ok/0172_record_field_list");
//     }
//     #[test]
//     fn test_0173_anonymous_const() {
//         check("parser/inline/ok/0173_anonymous_const");
//     }
//     #[test]
//     fn test_0173_macro_def_curly() {
//         check("parser/inline/ok/0173_macro_def_curly");
//     }
//     #[test]
//     fn test_0173_union_item() {
//         check("parser/inline/ok/0173_union_item");
//     }
//     #[test]
//     fn test_0174_trait_item_generic_params() {
//         check("parser/inline/ok/0174_trait_item_generic_params");
//     }
//     #[test]
//     fn test_0174_unit_struct() {
//         check("parser/inline/ok/0174_unit_struct");
//     }
//     #[test]
//     fn test_0174_use_tree_star() {
//         check("parser/inline/ok/0174_use_tree_star");
//     }
//     #[test]
//     fn test_0175_trait_item_bounds() {
//         check("parser/inline/ok/0175_trait_item_bounds");
//     }
//     #[test]
//     fn test_0176_trait_item_where_clause() {
//         check("parser/inline/ok/0176_trait_item_where_clause");
//     }
//     #[test]
//     fn test_0176_use_tree_alias() {
//         check("parser/inline/ok/0176_use_tree_alias");
//     }
//     #[test]
//     fn test_0177_assoc_item_list_inner_attrs() {
//         check("parser/inline/ok/0177_assoc_item_list_inner_attrs");
//     }
//     #[test]
//     fn test_0177_trait_alias_where_clause() {
//         check("parser/inline/ok/0177_trait_alias_where_clause");
//     }
//     #[test]
//     fn test_0177_use_tree() {
//         check("parser/inline/ok/0177_use_tree");
//     }
//     #[test]
//     fn test_0177_use_tree_path() {
//         check("parser/inline/ok/0177_use_tree_path");
//     }
//     #[test]
//     fn test_0178_use_tree_path_use_tree() {
//         check("parser/inline/ok/0178_use_tree_path_use_tree");
//     }
//     #[test]
//     fn test_0179_use_tree_abs_star() {
//         check("parser/inline/ok/0179_use_tree_abs_star");
//     }
//     #[test]
//     fn test_0180_use_tree_path_star() {
//         check("parser/inline/ok/0180_use_tree_path_star");
//     }
//     #[test]
//     fn test_0181_generic_param_attribute() {
//         check("parser/inline/ok/0181_generic_param_attribute");
//     }
//     #[test]
//     fn test_0181_use_item() {
//         check("parser/inline/ok/0181_use_item");
//     }
//     #[test]
//     fn test_0183_const_arg_block() {
//         check("parser/inline/ok/0183_const_arg_block");
//     }
//     #[test]
//     fn test_0183_type_param() {
//         check("parser/inline/ok/0183_type_param");
//     }
//     #[test]
//     fn test_0184_const_arg() {
//         check("parser/inline/ok/0184_const_arg");
//     }
//     #[test]
//     fn test_0184_generic_param_list() {
//         check("parser/inline/ok/0184_generic_param_list");
//     }
//     #[test]
//     fn test_0185_assoc_type_bound() {
//         check("parser/inline/ok/0185_assoc_type_bound");
//     }
//     #[test]
//     fn test_0187_assoc_type_eq() {
//         check("parser/inline/ok/0187_assoc_type_eq");
//     }
//     #[test]
//     fn test_0188_const_param_default_path() {
//         check("parser/inline/ok/0188_const_param_default_path");
//     }
//     #[test]
//     fn test_0189_const_arg_literal() {
//         check("parser/inline/ok/0189_const_arg_literal");
//     }
//     #[test]
//     fn test_0190_generic_arg() {
//         check("parser/inline/ok/0190_generic_arg");
//     }
//     #[test]
//     fn test_0191_const_arg_negative_number() {
//         check("parser/inline/ok/0191_const_arg_negative_number");
//     }
//     #[test]
//     fn test_0192_const_arg_bool_literal() {
//         check("parser/inline/ok/0192_const_arg_bool_literal");
//     }
//     #[test]
//     fn test_0193_let_stmt_init() {
//         check("parser/inline/ok/0193_let_stmt_init");
//     }
//     #[test]
//     fn test_0194_let_else() {
//         check("parser/inline/ok/0194_let_else");
//     }
//     #[test]
//     fn test_0194_let_stmt_ascription() {
//         check("parser/inline/ok/0194_let_stmt_ascription");
//     }
//     #[test]
//     fn test_0194_macro_inside_generic_arg() {
//         check("parser/inline/ok/0194_macro_inside_generic_arg");
//     }
//     #[test]
//     fn test_0196_pub_tuple_field() {
//         check("parser/inline/ok/0196_pub_tuple_field");
//     }
//     #[test]
//     fn test_0197_destructuring_assignment_struct_rest_pattern() {
//         check("parser/inline/ok/0197_destructuring_assignment_struct_rest_pattern");
//     }
//     #[test]
//     fn test_0198_destructuring_assignment_wildcard_pat() {
//         check("parser/inline/ok/0198_destructuring_assignment_wildcard_pat");
//     }
//     #[test]
//     fn test_0199_const_param_default_expression() {
//         check("parser/inline/ok/0199_const_param_default_expression");
//     }
//     #[test]
//     fn test_0199_effect_blocks() {
//         check("parser/inline/ok/0199_effect_blocks");
//     }
//     #[test]
//     fn test_0199_type_item_where_clause_deprecated() {
//         check("parser/inline/ok/0199_type_item_where_clause_deprecated");
//     }
//     #[test]
//     fn test_0200_assoc_const_eq() {
//         check("parser/inline/ok/0200_assoc_const_eq");
//     }
//     #[test]
//     fn test_0200_const_param_default_literal() {
//         check("parser/inline/ok/0200_const_param_default_literal");
//     }
//     #[test]
//     fn test_0201_question_for_type_trait_bound() {
//         check("parser/inline/ok/0201_question_for_type_trait_bound");
//     }
//     #[test]
//     fn test_0202_typepathfn_with_coloncolon() {
//         check("parser/inline/ok/0202_typepathfn_with_coloncolon");
//     }
//     #[test]
//     fn test_0203_closure_body_underscore_assignment() {
//         check("parser/inline/ok/0203_closure_body_underscore_assignment");
//     }
//     #[test]
//     fn test_0204_yeet_expr() {
//         check("parser/inline/ok/0204_yeet_expr");
//     }
//     #[test]
//     fn test_0205_const_closure() {
//         check("parser/inline/ok/0205_const_closure");
//     }
// }
