/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Provides the [`Metadata`] type — a structured, ordered, type-safe key-value
//! store backed by [`serde_json::Value`].

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A structured, ordered, type-safe key-value store for attaching arbitrary
/// annotations to domain objects.
///
/// `Metadata` is backed by a [`BTreeMap<String, Value>`] (ordered by key) and
/// provides a typed API via `serde` round-trips.  It is intentionally generic —
/// no domain-specific assumptions are baked in.
///
/// # Examples
///
/// ```rust
/// use qubit_metadata::Metadata;
///
/// let mut meta = Metadata::new();
/// meta.set("author", "alice");
/// meta.set("priority", 3_i64);
/// meta.set("reviewed", true);
///
/// let author: Option<String> = meta.get("author");
/// assert_eq!(author.as_deref(), Some("alice"));
///
/// let priority: Option<i64> = meta.get("priority");
/// assert_eq!(priority, Some(3));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Metadata(BTreeMap<String, Value>);

impl Metadata {
    /// Creates an empty `Metadata` instance.
    #[inline]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Returns `true` if there are no entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of key-value pairs.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the given key exists.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Retrieves and deserializes the value associated with `key`.
    ///
    /// Returns `None` if the key does not exist or if the stored value cannot
    /// be deserialized into `T`.
    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.0
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Returns a reference to the raw [`Value`] for `key`, or `None` if absent.
    #[inline]
    pub fn get_raw(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Serializes `value` and inserts it under `key`.
    ///
    /// Returns the previous raw [`Value`] if the key was already present, or
    /// `None` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if `value` cannot be serialized to a [`Value`].  In practice
    /// this only happens for types that are not serializable (e.g. custom
    /// `Serialize` impls that always return an error).
    pub fn set<T>(&mut self, key: impl Into<String>, value: T) -> Option<Value>
    where
        T: Serialize,
    {
        let json = serde_json::to_value(value)
            .expect("Metadata::set: value must be serializable to serde_json::Value");
        self.0.insert(key.into(), json)
    }

    /// Inserts a raw [`Value`] directly, bypassing serialization.
    ///
    /// Returns the previous value if present.
    #[inline]
    pub fn set_raw(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        self.0.insert(key.into(), value)
    }

    /// Removes the entry for `key` and returns the raw [`Value`] if it existed.
    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }

    /// Removes all entries.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over `(&str, &Value)` pairs in key-sorted order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Returns an iterator over the keys in sorted order.
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    /// Returns an iterator over the raw values in key-sorted order.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.0.values()
    }

    /// Merges all entries from `other` into `self`, overwriting existing keys.
    pub fn merge(&mut self, other: Metadata) {
        for (k, v) in other.0 {
            self.0.insert(k, v);
        }
    }

    /// Returns a new `Metadata` that contains all entries from both `self` and
    /// `other`.  Entries in `other` take precedence on key conflicts.
    #[must_use]
    pub fn merged(&self, other: &Metadata) -> Metadata {
        let mut result = self.clone();
        for (k, v) in &other.0 {
            result.0.insert(k.clone(), v.clone());
        }
        result
    }

    /// Retains only the entries for which `predicate` returns `true`.
    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&str, &Value) -> bool,
    {
        self.0.retain(|k, v| predicate(k.as_str(), v));
    }

    /// Converts this `Metadata` into its underlying [`BTreeMap`].
    #[inline]
    pub fn into_inner(self) -> BTreeMap<String, Value> {
        self.0
    }
}

impl From<BTreeMap<String, Value>> for Metadata {
    #[inline]
    fn from(map: BTreeMap<String, Value>) -> Self {
        Self(map)
    }
}

impl From<Metadata> for BTreeMap<String, Value> {
    #[inline]
    fn from(meta: Metadata) -> Self {
        meta.0
    }
}

impl FromIterator<(String, Value)> for Metadata {
    fn from_iter<I: IntoIterator<Item = (String, Value)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Metadata {
    type Item = (String, Value);
    type IntoIter = std::collections::btree_map::IntoIter<String, Value>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Metadata {
    type Item = (&'a String, &'a Value);
    type IntoIter = std::collections::btree_map::Iter<'a, String, Value>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Extend<(String, Value)> for Metadata {
    fn extend<I: IntoIterator<Item = (String, Value)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}
