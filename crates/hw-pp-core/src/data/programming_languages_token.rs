//! Syntax-highlighting tokens for hello-world snippets and their ANSI rendering.

use std::fmt::Write;

use crate::data::{programming_languages::ProgrammingLanguageVersion, theme::Theme};

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

impl ProgrammingLanguageVersion {
    /// Renders this version's tokens as ANSI-colored text in `theme`
    /// (`None` → [`Theme::Default`], i.e. Kanga).
    ///
    /// Print it with `anstream::println!` so colors are dropped when piped or `NO_COLOR` is set.
    #[must_use]
    pub fn highlighted(&self, theme: Option<Theme>) -> String {
        let mut collected = String::with_capacity(self.syntax.len());
        let resolved_theme = match theme {
            Some(existing_theme) => existing_theme,
            None => Theme::Default,
        };
        for &(token_kind, text) in self.tokenized_syntax {
            let style = token_kind.style(resolved_theme);
            // `{style}` / `{style:#}` render the style's ANSI start / reset codes. Writing
            // to a `String` never fails, so the `fmt::Result` is safe to ignore.
            let _ = write!(collected, "{style}{text}{style:#}");
        }
        collected
    }
}
