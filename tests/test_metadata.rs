/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Unit tests for [`qubit_metadata::Metadata`].

use std::collections::BTreeMap;

use serde_json::{json, Value};

use qubit_metadata::Metadata;

// ── Construction ─────────────────────────────────────────────────────────────

#[test]
fn new_is_empty() {
    let meta = Metadata::new();
    assert!(meta.is_empty());
    assert_eq!(meta.len(), 0);
}

#[test]
fn default_is_empty() {
    let meta = Metadata::default();
    assert!(meta.is_empty());
}

// ── set / get ────────────────────────────────────────────────────────────────

#[test]
fn set_and_get_string() {
    let mut meta = Metadata::new();
    meta.set("author", "alice");
    let v: Option<String> = meta.get("author");
    assert_eq!(v.as_deref(), Some("alice"));
}

#[test]
fn set_and_get_i64() {
    let mut meta = Metadata::new();
    meta.set("priority", 42_i64);
    let v: Option<i64> = meta.get("priority");
    assert_eq!(v, Some(42));
}

#[test]
fn set_and_get_bool() {
    let mut meta = Metadata::new();
    meta.set("reviewed", true);
    let v: Option<bool> = meta.get("reviewed");
    assert_eq!(v, Some(true));
}

#[test]
fn set_and_get_f64() {
    let mut meta = Metadata::new();
    meta.set("score", 3.14_f64);
    let v: Option<f64> = meta.get("score");
    assert!((v.unwrap() - 3.14).abs() < 1e-10);
}

#[test]
fn set_overwrites_previous_value() {
    let mut meta = Metadata::new();
    meta.set("key", "first");
    let old = meta.set("key", "second");
    assert_eq!(old, Some(json!("first")));
    let v: Option<String> = meta.get("key");
    assert_eq!(v.as_deref(), Some("second"));
}

#[test]
fn get_missing_key_returns_none() {
    let meta = Metadata::new();
    let v: Option<String> = meta.get("missing");
    assert!(v.is_none());
}

#[test]
fn get_wrong_type_returns_none() {
    let mut meta = Metadata::new();
    meta.set("key", "not-a-number");
    let v: Option<i64> = meta.get("key");
    assert!(v.is_none());
}

// ── get_raw / set_raw ────────────────────────────────────────────────────────

#[test]
fn get_raw_returns_value_ref() {
    let mut meta = Metadata::new();
    meta.set("x", 1_i64);
    assert_eq!(meta.get_raw("x"), Some(&json!(1)));
}

#[test]
fn set_raw_inserts_value() {
    let mut meta = Metadata::new();
    meta.set_raw("raw", json!({"nested": true}));
    assert_eq!(meta.get_raw("raw"), Some(&json!({"nested": true})));
}

// ── contains_key / len / is_empty ────────────────────────────────────────────

#[test]
fn contains_key() {
    let mut meta = Metadata::new();
    assert!(!meta.contains_key("k"));
    meta.set("k", "v");
    assert!(meta.contains_key("k"));
}

#[test]
fn len_tracks_entries() {
    let mut meta = Metadata::new();
    assert_eq!(meta.len(), 0);
    meta.set("a", 1_i64);
    assert_eq!(meta.len(), 1);
    meta.set("b", 2_i64);
    assert_eq!(meta.len(), 2);
    meta.set("a", 99_i64);
    assert_eq!(meta.len(), 2);
}

// ── remove / clear ───────────────────────────────────────────────────────────

#[test]
fn remove_existing_key() {
    let mut meta = Metadata::new();
    meta.set("k", "v");
    let removed = meta.remove("k");
    assert_eq!(removed, Some(json!("v")));
    assert!(!meta.contains_key("k"));
}

#[test]
fn remove_missing_key_returns_none() {
    let mut meta = Metadata::new();
    assert!(meta.remove("missing").is_none());
}

#[test]
fn clear_empties_metadata() {
    let mut meta = Metadata::new();
    meta.set("a", 1_i64);
    meta.set("b", 2_i64);
    meta.clear();
    assert!(meta.is_empty());
}

// ── Iteration ────────────────────────────────────────────────────────────────

#[test]
fn iter_returns_sorted_pairs() {
    let mut meta = Metadata::new();
    meta.set("z", "last");
    meta.set("a", "first");
    meta.set("m", "middle");

    let keys: Vec<&str> = meta.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["a", "m", "z"]);
}

#[test]
fn keys_iterator() {
    let mut meta = Metadata::new();
    meta.set("b", 1_i64);
    meta.set("a", 2_i64);
    let keys: Vec<&str> = meta.keys().collect();
    assert_eq!(keys, vec!["a", "b"]);
}

#[test]
fn values_iterator() {
    let mut meta = Metadata::new();
    meta.set("a", 1_i64);
    meta.set("b", 2_i64);
    let vals: Vec<&Value> = meta.values().collect();
    assert_eq!(vals, vec![&json!(1), &json!(2)]);
}

#[test]
fn into_iter_consumes_metadata() {
    let mut meta = Metadata::new();
    meta.set("x", 10_i64);
    let pairs: Vec<(String, Value)> = meta.into_iter().collect();
    assert_eq!(pairs, vec![("x".to_string(), json!(10))]);
}

#[test]
fn ref_into_iter() {
    let mut meta = Metadata::new();
    meta.set("k", "v");
    let count = (&meta).into_iter().count();
    assert_eq!(count, 1);
}

// ── merge / merged ───────────────────────────────────────────────────────────

#[test]
fn merge_adds_entries_from_other() {
    let mut a = Metadata::new();
    a.set("x", 1_i64);

    let mut b = Metadata::new();
    b.set("y", 2_i64);

    a.merge(b);
    assert_eq!(a.len(), 2);
    assert_eq!(a.get::<i64>("x"), Some(1));
    assert_eq!(a.get::<i64>("y"), Some(2));
}

#[test]
fn merge_overwrites_on_conflict() {
    let mut a = Metadata::new();
    a.set("k", "original");

    let mut b = Metadata::new();
    b.set("k", "overwritten");

    a.merge(b);
    assert_eq!(a.get::<String>("k").as_deref(), Some("overwritten"));
}

#[test]
fn merged_does_not_mutate_self() {
    let mut a = Metadata::new();
    a.set("x", 1_i64);

    let mut b = Metadata::new();
    b.set("y", 2_i64);

    let c = a.merged(&b);
    assert_eq!(a.len(), 1);
    assert_eq!(c.len(), 2);
}

// ── retain ───────────────────────────────────────────────────────────────────

#[test]
fn retain_keeps_matching_entries() {
    let mut meta = Metadata::new();
    meta.set("a", 1_i64);
    meta.set("b", 2_i64);
    meta.set("c", 3_i64);

    meta.retain(|k, _| k != "b");
    assert!(!meta.contains_key("b"));
    assert_eq!(meta.len(), 2);
}

// ── Conversions ──────────────────────────────────────────────────────────────

#[test]
fn from_btreemap() {
    let mut map = BTreeMap::new();
    map.insert("k".to_string(), json!("v"));
    let meta = Metadata::from(map);
    assert_eq!(meta.get::<String>("k").as_deref(), Some("v"));
}

#[test]
fn into_btreemap() {
    let mut meta = Metadata::new();
    meta.set("k", "v");
    let map: BTreeMap<String, Value> = meta.into();
    assert_eq!(map.get("k"), Some(&json!("v")));
}

#[test]
fn into_inner() {
    let mut meta = Metadata::new();
    meta.set("k", 1_i64);
    let inner = meta.into_inner();
    assert_eq!(inner.get("k"), Some(&json!(1)));
}

#[test]
fn from_iterator() {
    let pairs = vec![
        ("a".to_string(), json!(1)),
        ("b".to_string(), json!(2)),
    ];
    let meta: Metadata = pairs.into_iter().collect();
    assert_eq!(meta.len(), 2);
}

#[test]
fn extend() {
    let mut meta = Metadata::new();
    meta.set("a", 1_i64);
    meta.extend(vec![("b".to_string(), json!(2)), ("c".to_string(), json!(3))]);
    assert_eq!(meta.len(), 3);
}

// ── Serde round-trip ─────────────────────────────────────────────────────────

#[test]
fn serde_json_round_trip() {
    let mut meta = Metadata::new();
    meta.set("name", "bob");
    meta.set("age", 30_i64);
    meta.set("active", true);

    let json_str = serde_json::to_string(&meta).unwrap();
    let restored: Metadata = serde_json::from_str(&json_str).unwrap();
    assert_eq!(meta, restored);
}

#[test]
fn deserialize_from_json_object() {
    let json_str = r#"{"city":"Paris","population":2161000}"#;
    let meta: Metadata = serde_json::from_str(json_str).unwrap();
    assert_eq!(meta.get::<String>("city").as_deref(), Some("Paris"));
    assert_eq!(meta.get::<i64>("population"), Some(2_161_000));
}

// ── Clone / PartialEq ────────────────────────────────────────────────────────

#[test]
fn clone_is_independent() {
    let mut original = Metadata::new();
    original.set("k", "v");
    let mut cloned = original.clone();
    cloned.set("k", "changed");
    assert_eq!(original.get::<String>("k").as_deref(), Some("v"));
}

#[test]
fn partial_eq() {
    let mut a = Metadata::new();
    a.set("x", 1_i64);
    let mut b = Metadata::new();
    b.set("x", 1_i64);
    assert_eq!(a, b);

    b.set("x", 2_i64);
    assert_ne!(a, b);
}
