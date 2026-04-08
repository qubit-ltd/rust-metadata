/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # qubit-metadata
//!
//! A general-purpose, type-safe, extensible metadata model for Rust.
//!
//! This crate provides a `Metadata` type — a structured key-value store
//! designed for any domain that needs to attach arbitrary, typed, serializable
//! annotations to its data models. It is not a plain `HashMap` — it is a
//! structured extensibility point with type-safe access, `serde_json::Value`
//! backing, and first-class `serde` support.
//!
//! ## Design Goals
//!
//! - **Type Safety**: Typed get/set API backed by `serde_json::Value`
//! - **Generality**: No domain-specific assumptions — usable in any Rust project
//! - **Extensibility**: Acts as a structured extension point, not a stringly-typed bag
//! - **Serialization**: First-class `serde` support for JSON interchange
//! - **Filtering**: Optional `MetadataFilter` for composable query conditions
//!
//! ## Features
//!
//! - Core type: `Metadata` — an ordered key-value store with typed accessors
//! - Optional: `MetadataFilter` — composable filter expressions for metadata queries
//!
//! ## Example
//!
//! ```rust,ignore
//! use qubit_metadata::Metadata;
//!
//! let mut meta = Metadata::new();
//! meta.set("author", "alice");
//! meta.set("priority", 3_i64);
//!
//! let author: Option<String> = meta.get("author");
//! assert_eq!(author.as_deref(), Some("alice"));
//! ```
