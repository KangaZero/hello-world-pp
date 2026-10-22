//! The language/variant lookup error shared by both catalogs.

use thiserror::Error;

/// A failed language or variant lookup.
///
/// `L` is the catalog's language name type and `V` its variant type (a dialect or
/// a version).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LookupError<L, V> {
    /// The language has no catalog entry.
    #[error("{0} is not a recorded language")]
    UnknownLanguage(L),

    /// The language exists but does not record the variant.
    #[error("{variant} is not recorded for {language}")]
    UnknownVariant {
        /// The language that was searched.
        language: L,
        /// The variant it does not record.
        variant: V,
    },

    /// The language records no variants, so there is no default.
    #[error("{0} has no recorded dialects or versions to default to")]
    NoVariants(L),

    /// No recorded language owns the variant.
    #[error("no recorded language has {0}")]
    NoOwner(V),
}
