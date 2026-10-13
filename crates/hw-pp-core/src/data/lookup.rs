use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LookupError<L, V> {
    #[error("{0} is not a recorded language")]
    UnknownLanguage(L),

    #[error("{variant} is not recorded for {language}")]
    UnknownVariant { language: L, variant: V },

    #[error("{0} has no recorded dialects or versions to default to")]
    NoVariants(L),

    #[error("no recorded language has {0}")]
    NoOwner(V),
}
