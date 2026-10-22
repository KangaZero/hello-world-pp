//! Syntax-highlighting tokens and color themes for hello-world snippets.

use anstyle::{RgbColor, Style};
use std::fmt::Write;

use crate::data::{macros::name_enum, programming_languages::ProgrammingLanguageVersion};

/// What a highlighted slice of a snippet is, which picks its [`style`](TokenKind::style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// Identifiers, whitespace and anything else unhighlighted.
    Plain,
    /// Reserved words such as `fn` or `return`.
    Keyword,
    /// Type names.
    Type,
    /// Function names.
    Function,
    /// Macros and directives (e.g. `#include`, `.data`).
    Macro,
    /// String and character literals.
    String,
    /// Numeric literals.
    Number,
    /// Comments.
    Comment,
    /// Operators, brackets and separators.
    Punctuation,
    /// Code the language would reject.
    Invalid,
}

name_enum! {
    /// A color theme for [`collect_tokenized_syntax`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Themes: ParseThemeNameError("theme") {
        /// Same as [`Themes::Kanga`].
        Default => "Default",
        /// Tokyo Night Moon with a Dracula-purple accent.
        Kanga => "Kanga",
        /// Catppuccin's light flavor.
        CatppuccinLatte => "Catppuccin Latte" | "Latte",
        /// Catppuccin's lightest dark flavor.
        CatppuccinFrappe => "Catppuccin Frappe" | "Frappe",
        /// Catppuccin's medium dark flavor.
        CatppuccinMacchiato => "Catppuccin Macchiato" | "Macchiato",
        /// Catppuccin's darkest flavor.
        CatppuccinMocha => "Catppuccin Mocha" | "Mocha",
    }
}

/// A style with foreground color `r`, `g`, `b` and the default background.
#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> Style {
    RgbColor(r, g, b).on_default()
}

impl TokenKind {
    /// The style this kind of token is drawn with in `theme`.
    #[must_use]
    pub const fn style(self, theme: Themes) -> Style {
        match theme {
            Themes::Default | Themes::Kanga => match self {
                Self::Plain => rgb(200, 211, 245),
                Self::Punctuation => rgb(130, 139, 184),
                Self::Keyword => rgb(189, 147, 249).bold(),
                Self::Type => rgb(79, 214, 190),
                Self::Function => rgb(130, 170, 255),
                Self::Macro => rgb(252, 167, 234),
                Self::String => rgb(195, 232, 141),
                Self::Number => rgb(255, 150, 108),
                Self::Comment => rgb(99, 109, 166).italic(),
                Self::Invalid => rgb(255, 92, 87),
            },
            Themes::CatppuccinLatte => match self {
                Self::Plain => rgb(76, 79, 105),
                Self::Punctuation => rgb(124, 127, 147),
                Self::Keyword => rgb(136, 57, 239).bold(),
                Self::Type => rgb(223, 142, 29),
                Self::Function => rgb(30, 102, 245),
                Self::Macro => rgb(23, 146, 153),
                Self::String => rgb(64, 160, 43),
                Self::Number => rgb(254, 100, 11),
                Self::Comment => rgb(124, 127, 147).italic(),
                Self::Invalid => rgb(210, 15, 57),
            },
            Themes::CatppuccinFrappe => match self {
                Self::Plain => rgb(198, 208, 245),
                Self::Punctuation => rgb(148, 156, 187),
                Self::Keyword => rgb(202, 158, 230).bold(),
                Self::Type => rgb(229, 200, 144),
                Self::Function => rgb(140, 170, 238),
                Self::Macro => rgb(129, 200, 190),
                Self::String => rgb(166, 209, 137),
                Self::Number => rgb(239, 159, 118),
                Self::Comment => rgb(148, 156, 187).italic(),
                Self::Invalid => rgb(231, 130, 132),
            },
            Themes::CatppuccinMacchiato => match self {
                Self::Plain => rgb(202, 211, 245),
                Self::Punctuation => rgb(147, 154, 183),
                Self::Keyword => rgb(198, 160, 246).bold(),
                Self::Type => rgb(238, 212, 159),
                Self::Function => rgb(138, 173, 244),
                Self::Macro => rgb(139, 213, 202),
                Self::String => rgb(166, 218, 149),
                Self::Number => rgb(245, 169, 127),
                Self::Comment => rgb(147, 154, 183).italic(),
                Self::Invalid => rgb(237, 135, 150),
            },
            Themes::CatppuccinMocha => match self {
                Self::Plain => rgb(205, 214, 244),
                Self::Punctuation => rgb(147, 153, 178),
                Self::Keyword => rgb(203, 166, 247).bold(),
                Self::Type => rgb(249, 226, 175),
                Self::Function => rgb(137, 180, 250),
                Self::Macro => rgb(148, 226, 213),
                Self::String => rgb(166, 227, 161),
                Self::Number => rgb(250, 179, 135),
                Self::Comment => rgb(147, 153, 178).italic(),
                Self::Invalid => rgb(243, 139, 168),
            },
        }
    }
}

/// Renders `language_version`'s tokens as ANSI-colored text in `theme`
/// (`None` → [`Themes::Default`], i.e. Kanga).
///
/// Print it with `anstream::println!` so colors are dropped when piped or `NO_COLOR` is set.
#[must_use]
pub fn collect_tokenized_syntax(
    language_version: &ProgrammingLanguageVersion,
    theme: Option<Themes>,
) -> String {
    let mut collected = String::with_capacity(language_version.syntax.len());
    let resolved_theme = match theme {
        Some(existing_theme) => existing_theme,
        None => Themes::Default,
    };
    for &(token_kind, text) in language_version.tokenized_syntax {
        let style = token_kind.style(resolved_theme);
        // `{style}` / `{style:#}` render the style's ANSI start / reset codes. Writing
        // to a `String` never fails, so the `fmt::Result` is safe to ignore.
        let _ = write!(collected, "{style}{text}{style:#}");
    }
    collected
}

/// Display names of every theme, e.g. `"Kanga"`.
#[must_use]
pub fn get_all_themes_as_str() -> Vec<&'static str> {
    Themes::ALL.iter().map(|theme| theme.as_str()).collect()
}
