/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! [`MetadataValueType`] — JSON value classification for metadata.

use std::fmt;
use std::str::FromStr;

use serde_json::Value;

/// Coarse-grained JSON value types used by [`crate::MetadataError`] and inspection APIs.
///
/// `Metadata` stores arbitrary [`serde_json::Value`] instances, so it cannot
/// recover the caller's original Rust type. `MetadataValueType` is therefore a
/// JSON-level classification, analogous to the stricter `data_type()` concept
/// in `qubit-value`, but tailored to an open-ended JSON model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetadataValueType {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool,
    /// JSON number.
    Number,
    /// JSON string.
    String,
    /// JSON array.
    Array,
    /// JSON object.
    Object,
}

impl MetadataValueType {
    /// Returns the JSON value type of `value`.
    #[inline]
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(_) => Self::Bool,
            Value::Number(_) => Self::Number,
            Value::String(_) => Self::String,
            Value::Array(_) => Self::Array,
            Value::Object(_) => Self::Object,
        }
    }
}

impl From<&Value> for MetadataValueType {
    #[inline]
    fn from(value: &Value) -> Self {
        Self::of(value)
    }
}

impl fmt::Display for MetadataValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Null => "null",
            Self::Bool => "bool",
            Self::Number => "number",
            Self::String => "string",
            Self::Array => "array",
            Self::Object => "object",
        };
        f.write_str(name)
    }
}

impl FromStr for MetadataValueType {
    type Err = &'static str;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.eq_ignore_ascii_case("null") {
            Ok(Self::Null)
        } else if text.eq_ignore_ascii_case("bool") {
            Ok(Self::Bool)
        } else if text.eq_ignore_ascii_case("number") {
            Ok(Self::Number)
        } else if text.eq_ignore_ascii_case("string") {
            Ok(Self::String)
        } else if text.eq_ignore_ascii_case("array") {
            Ok(Self::Array)
        } else if text.eq_ignore_ascii_case("object") {
            Ok(Self::Object)
        } else {
            Err("invalid metadata value type")
        }
    }
}
