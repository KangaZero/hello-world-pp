use anstyle::{RgbColor, Style};
use std::fmt::Write;

use crate::data::{macros::name_enum, programming_languages::ProgrammingLanguageVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Plain,
    Keyword,
    Type,
    Function,
    Macro,
    String,
    Number,
    /// No current snippet with a bundled grammar contains a comment.
    #[allow(dead_code)]
    Comment,
    Punctuation,
}

name_enum! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Themes: ParseThemeNameError("theme") {
        Default => "Default", // Same as CatppuccinMocha
        CatppuccinLatte => "Catppuccin Latte",
        CatppuccinFrappe => "Catppuccin Frappe",
        CatppuccinMacchiato => "Catppuccin Macchiato",
        CatppuccinMocha => "Catppuccin Mocha",
    }
}

#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> Style {
    RgbColor(r, g, b).on_default()
}

impl TokenKind {
    const fn style(self, theme: Themes) -> Style {
        match theme {
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
            },
            Themes::Default | Themes::CatppuccinMocha => match self {
                Self::Plain => rgb(205, 214, 244),
                Self::Punctuation => rgb(147, 153, 178),
                Self::Keyword => rgb(203, 166, 247).bold(),
                Self::Type => rgb(249, 226, 175),
                Self::Function => rgb(137, 180, 250),
                Self::Macro => rgb(148, 226, 213),
                Self::String => rgb(166, 227, 161),
                Self::Number => rgb(250, 179, 135),
                Self::Comment => rgb(147, 153, 178).italic(),
            },
        }
    }
}

/// Renders a version's [`ProgrammingLanguageVersion::tokenized_syntax`] as one ANSI-styled
/// string, colouring each token by its [`TokenKind`] in the given theme.
///
/// `theme` defaults to [`Themes::Default`] (Catppuccin Mocha) when `None`.
///
/// The result always contains escape codes. Print it with `anstream::println!`
/// (or another ANSI-aware writer) so the colours are stripped when stdout is
/// not a terminal or `NO_COLOR` is set. With the styles removed, the text is
/// exactly [`ProgrammingLanguageVersion::syntax`].
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
        // Same as write!(collected, "{}{}{}", style.render(), text, style.render_reset());
        let _ = write!(collected, "{style}{text}{style:#}");
    }
    collected
}

#[must_use]
pub fn get_all_themes_as_str() -> Vec<&'static str> {
    Themes::ALL.iter().map(|theme| theme.as_str()).collect()
}
