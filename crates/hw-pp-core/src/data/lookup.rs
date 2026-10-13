use thiserror::Error;

/// Errors shared by every catalog lookup, generic over the catalog's language
/// name `L` and its variant `V` (a human dialect, or a programming language version).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LookupError<L, V> {
    /// The language has no entry in the catalog.
    #[error("{0} is not a recorded language")]
    UnknownLanguage(L),

    /// The language exists, but the requested variant is not one of its variants.
    #[error("{variant} is not recorded for {language}")]
    UnknownVariant { language: L, variant: V },

    /// The language has no variants, so no default can be chosen.
    #[error("{0} has no recorded dialects or versions to default to")]
    NoVariants(L),

    /// A variant was given on its own, but no recorded language owns it.
    #[error("no recorded language has {0}")]
    NoOwner(V),
}
