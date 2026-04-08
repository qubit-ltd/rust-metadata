/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Unit tests for [`qubit_metadata::MetadataFilter`] and [`qubit_metadata::Condition`].

use qubit_metadata::{Metadata, MetadataFilter};

fn sample() -> Metadata {
    let mut m = Metadata::new();
    m.set("status", "active");
    m.set("score", 42_i64);
    m.set("ratio", 0.75_f64);
    m.set("verified", true);
    m.set("tag", "rust");
    m
}

// ── Eq / Ne ──────────────────────────────────────────────────────────────────

#[test]
fn eq_matches_equal_string() {
    let f = MetadataFilter::eq("status", "active");
    assert!(f.matches(&sample()));
}

#[test]
fn eq_does_not_match_different_string() {
    let f = MetadataFilter::eq("status", "inactive");
    assert!(!f.matches(&sample()));
}

#[test]
fn eq_missing_key_does_not_match() {
    let f = MetadataFilter::eq("missing", "x");
    assert!(!f.matches(&sample()));
}

#[test]
fn ne_matches_different_value() {
    let f = MetadataFilter::ne("status", "inactive");
    assert!(f.matches(&sample()));
}

#[test]
fn ne_does_not_match_equal_value() {
    let f = MetadataFilter::ne("status", "active");
    assert!(!f.matches(&sample()));
}

#[test]
fn ne_missing_key_matches() {
    let f = MetadataFilter::ne("missing", "anything");
    assert!(f.matches(&sample()));
}

// ── Gt / Gte / Lt / Lte ──────────────────────────────────────────────────────

#[test]
fn gt_integer() {
    assert!(MetadataFilter::gt("score", 10_i64).matches(&sample()));
    assert!(!MetadataFilter::gt("score", 42_i64).matches(&sample()));
    assert!(!MetadataFilter::gt("score", 100_i64).matches(&sample()));
}

#[test]
fn gte_integer() {
    assert!(MetadataFilter::gte("score", 42_i64).matches(&sample()));
    assert!(MetadataFilter::gte("score", 10_i64).matches(&sample()));
    assert!(!MetadataFilter::gte("score", 43_i64).matches(&sample()));
}

#[test]
fn lt_integer() {
    assert!(MetadataFilter::lt("score", 100_i64).matches(&sample()));
    assert!(!MetadataFilter::lt("score", 42_i64).matches(&sample()));
    assert!(!MetadataFilter::lt("score", 10_i64).matches(&sample()));
}

#[test]
fn lte_integer() {
    assert!(MetadataFilter::lte("score", 42_i64).matches(&sample()));
    assert!(MetadataFilter::lte("score", 100_i64).matches(&sample()));
    assert!(!MetadataFilter::lte("score", 41_i64).matches(&sample()));
}

#[test]
fn gt_string_lexicographic() {
    assert!(MetadataFilter::gt("status", "aaa").matches(&sample()));
    assert!(!MetadataFilter::gt("status", "zzz").matches(&sample()));
}

#[test]
fn range_filter_missing_key_does_not_match() {
    assert!(!MetadataFilter::gt("missing", 0_i64).matches(&sample()));
    assert!(!MetadataFilter::gte("missing", 0_i64).matches(&sample()));
    assert!(!MetadataFilter::lt("missing", 100_i64).matches(&sample()));
    assert!(!MetadataFilter::lte("missing", 100_i64).matches(&sample()));
}

// ── Exists / NotExists ───────────────────────────────────────────────────────

#[test]
fn exists_present_key() {
    assert!(MetadataFilter::exists("status").matches(&sample()));
}

#[test]
fn exists_missing_key() {
    assert!(!MetadataFilter::exists("nope").matches(&sample()));
}

#[test]
fn not_exists_missing_key() {
    assert!(MetadataFilter::not_exists("nope").matches(&sample()));
}

#[test]
fn not_exists_present_key() {
    assert!(!MetadataFilter::not_exists("status").matches(&sample()));
}

// ── In / NotIn ───────────────────────────────────────────────────────────────

#[test]
fn in_values_matches() {
    let f = MetadataFilter::in_values("status", ["active", "pending"]);
    assert!(f.matches(&sample()));
}

#[test]
fn in_values_no_match() {
    let f = MetadataFilter::in_values("status", ["inactive", "pending"]);
    assert!(!f.matches(&sample()));
}

#[test]
fn in_values_missing_key() {
    let f = MetadataFilter::in_values("missing", ["x"]);
    assert!(!f.matches(&sample()));
}

#[test]
fn not_in_values_matches() {
    let f = MetadataFilter::not_in_values("status", ["inactive", "pending"]);
    assert!(f.matches(&sample()));
}

#[test]
fn not_in_values_no_match() {
    let f = MetadataFilter::not_in_values("status", ["active", "pending"]);
    assert!(!f.matches(&sample()));
}

#[test]
fn not_in_values_missing_key_matches() {
    let f = MetadataFilter::not_in_values("missing", ["x"]);
    assert!(f.matches(&sample()));
}

// ── And ──────────────────────────────────────────────────────────────────────

#[test]
fn and_all_true() {
    let f = MetadataFilter::eq("status", "active")
        .and(MetadataFilter::gte("score", 10_i64))
        .and(MetadataFilter::exists("verified"));
    assert!(f.matches(&sample()));
}

#[test]
fn and_one_false() {
    let f = MetadataFilter::eq("status", "active")
        .and(MetadataFilter::gt("score", 100_i64));
    assert!(!f.matches(&sample()));
}

#[test]
fn and_flattens_children() {
    let f = MetadataFilter::eq("status", "active")
        .and(MetadataFilter::exists("score"))
        .and(MetadataFilter::exists("tag"));
    if let MetadataFilter::And(children) = &f {
        assert_eq!(children.len(), 3);
    } else {
        panic!("expected And node");
    }
}

// ── Or ───────────────────────────────────────────────────────────────────────

#[test]
fn or_one_true() {
    let f = MetadataFilter::eq("status", "inactive")
        .or(MetadataFilter::eq("status", "active"));
    assert!(f.matches(&sample()));
}

#[test]
fn or_all_false() {
    let f = MetadataFilter::eq("status", "inactive")
        .or(MetadataFilter::eq("status", "pending"));
    assert!(!f.matches(&sample()));
}

#[test]
fn or_flattens_children() {
    let f = MetadataFilter::eq("status", "a")
        .or(MetadataFilter::eq("status", "b"))
        .or(MetadataFilter::eq("status", "active"));
    if let MetadataFilter::Or(children) = &f {
        assert_eq!(children.len(), 3);
    } else {
        panic!("expected Or node");
    }
}

// ── Not ──────────────────────────────────────────────────────────────────────

#[test]
fn not_inverts_true() {
    let f = MetadataFilter::eq("status", "active").not();
    assert!(!f.matches(&sample()));
}

#[test]
fn not_inverts_false() {
    let f = MetadataFilter::eq("status", "inactive").not();
    assert!(f.matches(&sample()));
}

// ── Complex compositions ─────────────────────────────────────────────────────

#[test]
fn complex_and_or_not() {
    // (status == "active" AND score >= 10) OR (NOT exists("nope"))
    let f = MetadataFilter::eq("status", "active")
        .and(MetadataFilter::gte("score", 10_i64))
        .or(MetadataFilter::exists("nope").not());
    assert!(f.matches(&sample()));
}

#[test]
fn empty_and_matches_everything() {
    let f = MetadataFilter::And(vec![]);
    assert!(f.matches(&sample()));
    assert!(f.matches(&Metadata::new()));
}

#[test]
fn empty_or_matches_nothing() {
    let f = MetadataFilter::Or(vec![]);
    assert!(!f.matches(&sample()));
    assert!(!f.matches(&Metadata::new()));
}

// ── Serde round-trip ─────────────────────────────────────────────────────────

#[test]
fn filter_serde_round_trip() {
    let f = MetadataFilter::eq("status", "active")
        .and(MetadataFilter::gte("score", 10_i64))
        .or(MetadataFilter::exists("tag").not());

    let json = serde_json::to_string(&f).unwrap();
    let restored: MetadataFilter = serde_json::from_str(&json).unwrap();
    assert_eq!(f, restored);
    assert_eq!(f.matches(&sample()), restored.matches(&sample()));
}
