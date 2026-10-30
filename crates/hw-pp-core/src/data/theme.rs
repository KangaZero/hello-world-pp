//! Syntax-highlighting color themes and the style of each token kind in them.

use anstyle::{RgbColor, Style};

use crate::data::{macros::name_enum, programming_languages_token::TokenKind};

name_enum! {
    /// A color theme for [`highlighted`] snippets.
    ///
    /// [`highlighted`]: super::programming_languages::ProgrammingLanguageVersion::highlighted
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub enum Theme: ParseThemeNameError("theme") {
        /// Same as [`Theme::Kanga`].
        #[default]
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

/// A theme's colors outside of code tokens: CLI help, errors, warnings and the banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThemeColors {
    /// Section headers in help, e.g. `Options:`, `Commands:`.
    pub header: Style,
    /// The `Usage:` label.
    pub usage: Style,
    /// Literal flags and subcommands in help, e.g. `--color`, `list`.
    pub literal: Style,
    /// Value placeholders in help, e.g. `<LANGUAGE>`.
    pub placeholder: Style,
    /// `error:` labels.
    pub error: Style,
    /// Suggested valid values in error messages.
    pub valid: Style,
    /// The rejected input in error messages.
    pub invalid: Style,
    /// `warning:` labels: the palette's yellow, bold.
    pub warning: Style,
    /// The ASCII-art banner.
    pub banner: Style,
}
//
// impl Default for Theme {
//     fn default() -> Self {
//         Self::Default
//     }
// }

impl Theme {
    /// This theme's [`ThemeColors`], from the same palette as its snippet colors.
    #[must_use]
    pub const fn colors(self) -> ThemeColors {
        match self {
            // Tokyo Night Moon; purple keyword accent
            Self::Default | Self::Kanga => ThemeColors {
                header: rgb(189, 147, 249).bold(),
                usage: rgb(189, 147, 249).bold(),
                literal: rgb(130, 170, 255).bold(),
                placeholder: rgb(79, 214, 190),
                error: rgb(255, 92, 87).bold(),
                valid: rgb(195, 232, 141),
                invalid: rgb(255, 150, 108).bold(),
                warning: rgb(255, 199, 119).bold(),
                banner: rgb(189, 147, 249).bold(),
            },
            // Catppuccin Latte
            Self::CatppuccinLatte => ThemeColors {
                header: rgb(136, 57, 239).bold(),
                usage: rgb(136, 57, 239).bold(),
                literal: rgb(30, 102, 245).bold(),
                placeholder: rgb(223, 142, 29),
                error: rgb(210, 15, 57).bold(),
                valid: rgb(64, 160, 43),
                invalid: rgb(254, 100, 11).bold(),
                warning: rgb(223, 142, 29).bold(),
                banner: rgb(136, 57, 239).bold(),
            },
            // Catppuccin Frappé
            Self::CatppuccinFrappe => ThemeColors {
                header: rgb(202, 158, 230).bold(),
                usage: rgb(202, 158, 230).bold(),
                literal: rgb(140, 170, 238).bold(),
                placeholder: rgb(229, 200, 144),
                error: rgb(231, 130, 132).bold(),
                valid: rgb(166, 209, 137),
                invalid: rgb(239, 159, 118).bold(),
                warning: rgb(229, 200, 144).bold(),
                banner: rgb(202, 158, 230).bold(),
            },
            // Catppuccin Macchiato
            Self::CatppuccinMacchiato => ThemeColors {
                header: rgb(198, 160, 246).bold(),
                usage: rgb(198, 160, 246).bold(),
                literal: rgb(138, 173, 244).bold(),
                placeholder: rgb(238, 212, 159),
                error: rgb(237, 135, 150).bold(),
                valid: rgb(166, 218, 149),
                invalid: rgb(245, 169, 127).bold(),
                warning: rgb(238, 212, 159).bold(),
                banner: rgb(198, 160, 246).bold(),
            },
            // Catppuccin Mocha
            Self::CatppuccinMocha => ThemeColors {
                header: rgb(203, 166, 247).bold(),
                usage: rgb(203, 166, 247).bold(),
                literal: rgb(137, 180, 250).bold(),
                placeholder: rgb(249, 226, 175),
                error: rgb(243, 139, 168).bold(),
                valid: rgb(166, 227, 161),
                invalid: rgb(250, 179, 135).bold(),
                warning: rgb(249, 226, 175).bold(),
                banner: rgb(203, 166, 247).bold(),
            },
        }
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
    pub const fn style(self, theme: Theme) -> Style {
        match theme {
            Theme::Default | Theme::Kanga => match self {
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
            Theme::CatppuccinLatte => match self {
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
            Theme::CatppuccinFrappe => match self {
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
            Theme::CatppuccinMacchiato => match self {
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
            Theme::CatppuccinMocha => match self {
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
