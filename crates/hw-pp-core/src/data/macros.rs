//! Declarative macros shared by the catalogs and language files.

/// Defines a fieldless enum from one `Variant => "Display Name"` list (or
/// `Variant => "Display Name" | "Endonym"`), plus `ALL`, `as_str`, `endonym`,
/// `Display`, `FromStr` and a parse error type.
///
/// The second name is an endonym for languages and dialects, and a short alias
/// elsewhere (`"UTF 8"`, `"Latte"`).
///
/// `FromStr` trims its input and accepts display names and endonyms, ignoring
/// case, then accents and `-`/`_` versus spaces (via `folded_eq`, without allocating) as a
/// fallback.
///
/// ```ignore
/// name_enum! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub enum Color: ParseColorError("color") { DarkRed => "Dark Red" | "Rouge Foncé" }
/// }
/// assert_eq!(" dark red ".parse(), Ok(Color::DarkRed));
/// assert_eq!("rouge fonce".parse(), Ok(Color::DarkRed));
/// assert_eq!(Color::DarkRed.to_string(), "Dark Red");
/// assert_eq!(Color::DarkRed.endonym(), Some("Rouge Foncé"));
/// ```
macro_rules! name_enum {
    // Internal: optional endonym as `Option<&'static str>`.
    (@endonym) => { None };
    (@endonym $endonym:literal) => { Some($endonym) };
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident : $error:ident($kind:literal) {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $text:literal $(| $endonym:literal)?
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Every variant, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// The display name, as printed by `Display` and accepted by `FromStr`.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text,)+
                }
            }

            /// The second accepted name, if any: an endonym or a short alias.
            #[must_use]
            pub const fn endonym(self) -> Option<&'static str> {
                match self {
                    $(Self::$variant => name_enum!(@endonym $($endonym)?),)+
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        #[doc = concat!("Error returned when a string does not match any [`", stringify!($name), "`].")]
        #[derive(Debug, Clone, PartialEq, Eq, ::thiserror::Error)]
        #[error("unrecognized {} name: {:?}", $kind, .0)]
        $vis struct $error(String);

        impl $error {
            /// The input that failed to parse.
            #[must_use]
            pub fn input(&self) -> &str {
                &self.0
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let trimmed = s.trim();
                // Each variant's display name, plus its endonym if it has one.
                let names = |variant: Self| ::core::iter::once(variant.as_str()).chain(variant.endonym());
                Self::ALL
                    .iter()
                    .copied()
                    .find(|&variant| names(variant).any(|name| name.eq_ignore_ascii_case(trimmed)))
                    .or_else(|| {
                        Self::ALL
                            .iter()
                            .copied()
                            .find(|&variant| names(variant).any(|name| $crate::data::normalize::folded_eq(name, trimmed)))
                    })
                    .ok_or_else(|| $error(s.to_owned()))
            }
        }
    };
}

/// `"Hello, World!"` as a literal (or the literal given), for use inside [`concat!`].
///
/// Only for snippets that print the message from a string literal; Brainfuck and
/// Befunge-93 encode it differently and keep it hand-written.
///
/// ```ignore
/// concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#) // printf("Hello, World!\n");
/// ```
macro_rules! output_for_hello_world {
    () => {
        "Hello, World!"
    };
    ($hello_world:literal) => {
        $hello_world
    };
}

/// Declares the enum variants a language file adds; expands to nothing.
///
/// `build.rs` reads it from every `data/<catalog>/<shard>/<name>.rs` (`<shard>` is
/// the first letter of the display name, or `#`) and generates the enums and table,
/// so adding a language means adding one file.
///
/// - Every pair is `Variant => "Name"`, optionally `| "Endonym"`.
/// - `language`: exactly one pair.
/// - Programming: `versions` (non-empty), `extensions` (may be empty).
/// - Human: `dialects` (non-empty). Scripts are not declared here: pick them from the
///   hand-written [`ScriptName`](super::human_languages::ScriptName).
/// - `extensions` may repeat across files with the same names; all else is unique.
///
/// ```ignore
/// language_names! {
///     language: Rust => "Rust",
///     versions: [Rust2021 => "Rust 2021"],
///     extensions: [Rs => "rs"],
/// }
/// ```
macro_rules! language_names {
    ($($declaration:tt)*) => {};
}

pub(crate) use language_names;
pub(crate) use name_enum;
pub(crate) use output_for_hello_world;
