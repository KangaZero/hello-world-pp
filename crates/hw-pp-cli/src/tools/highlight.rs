//! Syntax highlighting for hello-world snippets.
//!
//! Tokenizing happens at build time (see `build.rs`); this module only maps
//! each token to a terminal style and renders it.

use std::borrow::Cow;
use std::fmt::Write as _;

use anstyle::{AnsiColor, Style};
use hw_pp_core::data::common::remove_punctuation_from_hello_world;
use hw_pp_core::data::programming_languages::{LanguageVersion, ProgrammingLanguageDialect};

/// The kind of a highlighted piece of source code.
///
/// `build.rs` emits these by name, so every name it can produce must exist here
/// (a missing one is a compile error in the generated file).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TokenKind {
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

impl TokenKind {
    const fn style(self) -> Style {
        match self {
            Self::Plain | Self::Punctuation => Style::new(),
            Self::Keyword => AnsiColor::Magenta.on_default().bold(),
            Self::Type => AnsiColor::Yellow.on_default(),
            Self::Function => AnsiColor::Blue.on_default(),
            Self::Macro => AnsiColor::Cyan.on_default(),
            Self::String => AnsiColor::Green.on_default(),
            Self::Number => AnsiColor::BrightRed.on_default(),
            Self::Comment => AnsiColor::BrightBlack.on_default().italic(),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/highlighted.rs"));

/// The build-time tokens for `dialect`, if `build.rs` recorded any.
fn tokens_for(dialect: ProgrammingLanguageDialect) -> Option<&'static [(TokenKind, &'static str)]> {
    HIGHLIGHTED
        .iter()
        .find(|(label, _)| *label == dialect)
        .map(|(_, tokens)| *tokens)
}

/// Renders `version.syntax` with ANSI colours.
///
/// With `no_punctuation`, punctuation is stripped from string literals only,
/// so the code itself (`;`, `(`, `!` in `println!`) stays valid.
///
/// Print the result with `anstream::println!`, which drops the colours when
/// stdout is not a terminal or `NO_COLOR` is set.
pub(crate) fn render_highlighted(version: &LanguageVersion, no_punctuation: bool) -> String {
    let Some(tokens) = tokens_for(version.label) else {
        return version.syntax.to_owned();
    };
    let mut rendered = String::with_capacity(version.syntax.len());
    for &(kind, text) in tokens {
        let text: Cow<'_, str> = if no_punctuation && kind == TokenKind::String {
            Cow::Owned(remove_punctuation_from_hello_world(text))
        } else {
            Cow::Borrowed(text)
        };
        let style = kind.style();
        // Writing to a `String` cannot fail.
        let _ = write!(rendered, "{style}{text}{style:#}");
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;
    use hw_pp_core::data::programming_languages::PROGRAMMING_LANGUAGES;

    #[test]
    fn every_version_has_tokens_that_rebuild_its_snippet() {
        for version in PROGRAMMING_LANGUAGES.iter().flat_map(|lang| lang.versions) {
            let tokens = tokens_for(version.label)
                .unwrap_or_else(|| panic!("{} has no build-time tokens", version.label));
            let rebuilt: String = tokens.iter().map(|(_, text)| *text).collect();
            assert_eq!(
                rebuilt, version.syntax,
                "{} tokens lose text",
                version.label
            );
        }
    }

    #[test]
    fn rust_snippet_is_classified() {
        let tokens = tokens_for(ProgrammingLanguageDialect::Rust2021).expect("Rust has tokens");
        assert!(tokens.contains(&(TokenKind::Keyword, "fn")));
        assert!(tokens.contains(&(TokenKind::Macro, "println!")));
        assert!(tokens.contains(&(TokenKind::String, "\"Hello, World!\"")));
    }
}
