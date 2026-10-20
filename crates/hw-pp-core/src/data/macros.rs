/// Defines a fieldless enum from one `Variant => "Display Name"` list, plus `ALL`,
/// `as_str`, `Display`, case-insensitive `FromStr` and a parse error type.
///
/// ```ignore
/// name_enum! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub enum Color: ParseColorError("color") { DarkRed => "Dark Red" }
/// }
/// assert_eq!(" dark red ".parse(), Ok(Color::DarkRed));
/// assert_eq!(Color::DarkRed.to_string(), "Dark Red");
/// ```
macro_rules! name_enum {
       // Optional endonym Option<&'static str>
       (@endonym) => { None };
       (@endonym $endonym:literal) => { Some($endonym)};
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
            let normalized = super::normalize::normalize_text(trimmed);
            Self::ALL
                .iter()
                .copied()
                .find(|&variant| names(variant).any(|name| super::normalize::normalize_text(name) == normalized))
        })
        .ok_or_else(|| $error(s.to_owned()))
           }
        }
    };
}

/// `"Hello, World!"` as a literal (or the literal given), for use inside [`concat!`].
///
/// Only for snippets that print the message from a string literal; Brainfuck and
/// Befunge encode it differently and keep it hand-written.
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
/// `build.rs` reads it from every `data/<catalog>/<letter>/<name>.rs` and generates
/// the enums and table, so adding a language means adding one file.
///
/// - `language`: exactly one `Variant => "Name"`.
/// - Programming: `versions`, `extensions`. Human: `dialects`, `scripts` (may be empty).
/// - `extensions`/`scripts` may repeat across files with the same name; all else is unique.
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
