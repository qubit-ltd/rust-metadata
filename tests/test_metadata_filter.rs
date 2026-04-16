/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Unit tests for [`qubit_metadata::MetadataFilter`] combinators (`and`, `or`,
//! `not`) and full-tree serde. Leaf [`qubit_metadata::Condition`] tests live
//! in `test_condition.rs`.

use qubit_metadata::{Metadata, MetadataFilter};
use serde::{Serialize, Serializer};

struct FailingSerialize;

impl Serialize for FailingSerialize {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom(
            "intentional serialization failure",
        ))
    }
}

fn sample() -> Metadata {
    let mut m = Metadata::new();
    m.set("status", "active");
    m.set("score", 42_i64);
    m.set("ratio", 0.75_f64);
    m.set("verified", true);
    m.set("tag", "rust");
    m
}

// ── And ──────────────────────────────────────────────────────────────────────

#[test]
fn and_all_true() {
    let f = MetadataFilter::equal("status", "active")
        .unwrap()
        .and(MetadataFilter::greater_equal("score", 10_i64).unwrap())
        .and(MetadataFilter::exists("verified"));
    assert!(f.matches(&sample()));
}

#[test]
fn and_one_false() {
    let f = MetadataFilter::equal("status", "active")
        .unwrap()
        .and(MetadataFilter::greater("score", 100_i64).unwrap());
    assert!(!f.matches(&sample()));
}

#[test]
fn and_flattens_children() {
    let f = MetadataFilter::equal("status", "active")
        .unwrap()
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
    let f = MetadataFilter::equal("status", "inactive")
        .unwrap()
        .or(MetadataFilter::equal("status", "active").unwrap());
    assert!(f.matches(&sample()));
}

#[test]
fn or_all_false() {
    let f = MetadataFilter::equal("status", "inactive")
        .unwrap()
        .or(MetadataFilter::equal("status", "pending").unwrap());
    assert!(!f.matches(&sample()));
}

#[test]
fn or_flattens_children() {
    let f = MetadataFilter::equal("status", "a")
        .unwrap()
        .or(MetadataFilter::equal("status", "b").unwrap())
        .or(MetadataFilter::equal("status", "active").unwrap());
    if let MetadataFilter::Or(children) = &f {
        assert_eq!(children.len(), 3);
    } else {
        panic!("expected Or node");
    }
}

// ── Not ──────────────────────────────────────────────────────────────────────

#[test]
fn not_inverts_true() {
    let f = MetadataFilter::equal("status", "active").unwrap().not();
    assert!(!f.matches(&sample()));
}

#[test]
fn not_inverts_false() {
    let f = MetadataFilter::equal("status", "inactive").unwrap().not();
    assert!(f.matches(&sample()));
}

// ── Complex compositions ─────────────────────────────────────────────────────

#[test]
fn complex_and_or_not() {
    // (status == "active" AND score >= 10) OR (NOT exists("nope"))
    let f = MetadataFilter::equal("status", "active")
        .unwrap()
        .and(MetadataFilter::greater_equal("score", 10_i64).unwrap())
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

// ── Serde (MetadataFilter tree) ──────────────────────────────────────────────

#[test]
fn filter_serde_round_trip() {
    let f = MetadataFilter::equal("status", "active")
        .unwrap()
        .and(MetadataFilter::greater_equal("score", 10_i64).unwrap())
        .or(MetadataFilter::exists("tag").not());

    let json = serde_json::to_string(&f).unwrap();
    let restored: MetadataFilter = serde_json::from_str(&json).unwrap();
    assert_eq!(f, restored);
    assert_eq!(f.matches(&sample()), restored.matches(&sample()));
}

#[test]
fn leaf_constructor_reports_serialization_error_instead_of_panicking() {
    let result = MetadataFilter::equal("broken", FailingSerialize);
    assert!(result.is_err());
}

#[test]
fn set_constructor_reports_serialization_error_instead_of_panicking() {
    let result = MetadataFilter::in_values("broken", vec![FailingSerialize]);
    assert!(result.is_err());
}
