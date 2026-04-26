/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Composable metadata filters: [`MetadataFilter`], wire types, and builder.
mod missing_key_policy;
mod number_comparison_policy;
mod filter_match_options;
mod condition_wire;
mod filter_expr;
mod filter_expr_wire;
mod legacy_filter_expr;
mod legacy_metadata_filter_wire;
mod metadata_filter;
mod metadata_filter_builder;
mod metadata_filter_input;
mod metadata_filter_wire;

pub use filter_match_options::FilterMatchOptions;
pub use missing_key_policy::MissingKeyPolicy;
pub use number_comparison_policy::NumberComparisonPolicy;
pub use metadata_filter::MetadataFilter;
pub use metadata_filter_builder::MetadataFilterBuilder;
