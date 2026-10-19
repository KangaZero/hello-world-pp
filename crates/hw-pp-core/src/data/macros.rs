/// Defines a fieldless enum whose variants each map to one display name.
///
/// From a single `Variant => "Display Name"` list this generates:
/// - the enum itself (attributes and doc comments pass through),
/// - `ALL`, every variant in declaration order,
/// - `as_str`, the display name,
/// - [`core::fmt::Display`], writing `as_str`,
/// - [`core::str::FromStr`], matching a display name case-insensitively
///   (ASCII only) after trimming whitespace,
/// - a parse error type holding the rejected input.
///
/// ```ignore
/// name_enum! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub enum Color: ParseColorError("color") {
///         DarkRed => "Dark Red",
///     }
/// }
/// assert_eq!("dark red".parse::<Color>(), Ok(Color::DarkRed));
/// ```
macro_rules! name_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident : $error:ident($kind:literal) {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $text:literal
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

            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text,)+
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
                let needle = s.trim();
                Self::ALL
                    .iter()
                    .copied()
                    .find(|variant| variant.as_str().eq_ignore_ascii_case(needle))
                    .ok_or_else(|| $error(s.to_owned()))
            }
        }
    };
}

macro_rules! output_for_hello_world {
    () => {
        "Hello, World!"
    };
    ($hello_world:literal) => {
        $hello_world
    };
}

pub(crate) use name_enum;
pub(crate) use output_for_hello_world;
