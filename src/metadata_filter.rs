/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Provides [`MetadataFilter`] — composable filter expressions for
//! metadata-based queries.
//!
//! A [`MetadataFilter`] can be used to select [`Metadata`] instances that
//! satisfy a set of conditions.  Conditions can be combined with logical
//! operators (`and`, `or`, `not`) to form arbitrarily complex predicates.
//!
//! # Examples
//!
//! ```rust
//! use qubit_metadata::{Metadata, MetadataFilter};
//!
//! let mut meta = Metadata::new();
//! meta.set("status", "active");
//! meta.set("score", 42_i64);
//!
//! let filter = MetadataFilter::equal("status", "active")
//!     .and(MetadataFilter::greater_equal("score", 10_i64));
//!
//! assert!(filter.matches(&meta));
//! ```

use serde::{Deserialize, Serialize};

use crate::{Condition, Metadata};

/// Policy that controls how filters treat missing keys for negative predicates.
///
/// The policy only affects [`Condition::NotEqual`] and [`Condition::NotIn`].
/// Other predicates keep their existing semantics (`equal` requires presence,
/// `exists` / `not_exists` check presence directly, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MissingKeyPolicy {
    /// Missing keys satisfy negative predicates (`not_equal`, `not_in_values`).
    ///
    /// This is the historical behavior and therefore the default.
    #[default]
    Match,
    /// Missing keys do not satisfy negative predicates.
    NoMatch,
}

impl MissingKeyPolicy {
    #[inline]
    pub(crate) const fn matches_negative_predicates(self) -> bool {
        matches!(self, Self::Match)
    }
}

/// A composable filter expression over [`Metadata`].
///
/// Filters can be built from primitive [`Condition`]s and combined with
/// [`MetadataFilter::and`], [`MetadataFilter::or`], and [`MetadataFilter::not`].
///
/// # Examples
///
/// ```rust
/// use qubit_metadata::{Metadata, MetadataFilter};
///
/// let mut meta = Metadata::new();
/// meta.set("env", "prod");
/// meta.set("version", 2_i64);
///
/// let f = MetadataFilter::equal("env", "prod")
///     .and(MetadataFilter::greater_equal("version", 1_i64));
///
/// assert!(f.matches(&meta));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetadataFilter {
    /// A leaf condition.
    Condition(Condition),
    /// All child filters must match.
    And(Vec<MetadataFilter>),
    /// At least one child filter must match.
    Or(Vec<MetadataFilter>),
    /// The child filter must not match.
    Not(Box<MetadataFilter>),
}

impl MetadataFilter {
    // ── Leaf constructors ────────────────────────────────────────────────────

    /// Creates an equality filter: `key == value`.
    #[inline]
    pub fn equal<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::Equal {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::equal: value must be serializable"),
        })
    }

    /// Creates a not-equal filter: `key != value`.
    #[inline]
    pub fn not_equal<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::NotEqual {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::not_equal: value must be serializable"),
        })
    }

    /// Creates a greater-than filter: `key > value`.
    #[inline]
    pub fn greater<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::Greater {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::greater: value must be serializable"),
        })
    }

    /// Creates a greater-than-or-equal filter: `key >= value`.
    #[inline]
    pub fn greater_equal<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::GreaterEqual {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::greater_equal: value must be serializable"),
        })
    }

    /// Creates a less-than filter: `key < value`.
    #[inline]
    pub fn less<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::Less {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::less: value must be serializable"),
        })
    }

    /// Creates a less-than-or-equal filter: `key <= value`.
    #[inline]
    pub fn less_equal<T: Serialize>(key: impl Into<String>, value: T) -> Self {
        Self::Condition(Condition::LessEqual {
            key: key.into(),
            value: serde_json::to_value(value)
                .expect("MetadataFilter::less_equal: value must be serializable"),
        })
    }

    /// Creates an existence filter: the key must be present.
    #[inline]
    pub fn exists(key: impl Into<String>) -> Self {
        Self::Condition(Condition::Exists { key: key.into() })
    }

    /// Creates a non-existence filter: the key must be absent.
    #[inline]
    pub fn not_exists(key: impl Into<String>) -> Self {
        Self::Condition(Condition::NotExists { key: key.into() })
    }

    /// Creates an in-set filter: `key ∈ values`.
    #[inline]
    pub fn in_values<T, I>(key: impl Into<String>, values: I) -> Self
    where
        T: Serialize,
        I: IntoIterator<Item = T>,
    {
        let values = values
            .into_iter()
            .map(|v| {
                serde_json::to_value(v)
                    .expect("MetadataFilter::in_values: each value must be serializable")
            })
            .collect();
        Self::Condition(Condition::In {
            key: key.into(),
            values,
        })
    }

    /// Creates a not-in-set filter: `key ∉ values`.
    #[inline]
    pub fn not_in_values<T, I>(key: impl Into<String>, values: I) -> Self
    where
        T: Serialize,
        I: IntoIterator<Item = T>,
    {
        let values = values
            .into_iter()
            .map(|v| {
                serde_json::to_value(v)
                    .expect("MetadataFilter::not_in_values: each value must be serializable")
            })
            .collect();
        Self::Condition(Condition::NotIn {
            key: key.into(),
            values,
        })
    }

    // ── Logical combinators ──────────────────────────────────────────────────

    /// Combines `self` and `other` with a logical AND.
    ///
    /// If `self` is already an `And` node the new filter is appended to its
    /// children rather than creating a new nested node.
    #[inline]
    #[must_use]
    pub fn and(self, other: MetadataFilter) -> Self {
        match self {
            MetadataFilter::And(mut children) => {
                children.push(other);
                MetadataFilter::And(children)
            }
            _ => MetadataFilter::And(vec![self, other]),
        }
    }

    /// Combines `self` and `other` with a logical OR.
    ///
    /// If `self` is already an `Or` node the new filter is appended to its
    /// children rather than creating a new nested node.
    #[inline]
    #[must_use]
    pub fn or(self, other: MetadataFilter) -> Self {
        match self {
            MetadataFilter::Or(mut children) => {
                children.push(other);
                MetadataFilter::Or(children)
            }
            _ => MetadataFilter::Or(vec![self, other]),
        }
    }

    /// Wraps `self` in a logical NOT.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    #[must_use]
    pub fn not(self) -> Self {
        !self
    }

    // ── Evaluation ───────────────────────────────────────────────────────────

    /// Returns `true` if `meta` satisfies this filter.
    #[inline]
    pub fn matches(&self, meta: &Metadata) -> bool {
        self.matches_with_missing_key_policy(meta, MissingKeyPolicy::default())
    }

    /// Returns `true` if `meta` satisfies this filter using `missing_key_policy`.
    ///
    /// This policy only affects negative predicates that can be interpreted in
    /// two ways when the key is absent: [`MetadataFilter::not_equal`] and
    /// [`MetadataFilter::not_in_values`].
    #[inline]
    pub fn matches_with_missing_key_policy(
        &self,
        meta: &Metadata,
        missing_key_policy: MissingKeyPolicy,
    ) -> bool {
        match self {
            MetadataFilter::Condition(cond) => cond.matches(meta, missing_key_policy),
            MetadataFilter::And(children) => children
                .iter()
                .all(|f| f.matches_with_missing_key_policy(meta, missing_key_policy)),
            MetadataFilter::Or(children) => children
                .iter()
                .any(|f| f.matches_with_missing_key_policy(meta, missing_key_policy)),
            MetadataFilter::Not(inner) => {
                !inner.matches_with_missing_key_policy(meta, missing_key_policy)
            }
        }
    }
}

impl std::ops::Not for MetadataFilter {
    type Output = MetadataFilter;

    #[inline]
    fn not(self) -> Self::Output {
        MetadataFilter::Not(Box::new(self))
    }
}
