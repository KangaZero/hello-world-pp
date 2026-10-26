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

/// Each token's kind and byte length, after checking that the tokens' text concatenates
/// exactly to `syntax`.
///
/// Call it in a `const` block, so a mismatch is a compile error naming the language file.
///
/// # Panics
///
/// If the tokens overrun, differ from, or do not cover all of `syntax`.
#[must_use]
pub(crate) const fn checked_token_lengths<const N: usize>(
    syntax: &str,
    tokens: [(TokenKind, &str); N],
) -> [(TokenKind, usize); N] {
    let syntax = syntax.as_bytes();
    let mut lengths = [(TokenKind::Plain, 0); N];
    let mut offset = 0;
    let mut index = 0;
    // `while`, not iterators: `const fn` cannot call trait methods.
    while index < N {
        let (kind, text) = tokens[index];
        let text = text.as_bytes();
        assert!(
            offset + text.len() <= syntax.len(),
            "tokens run past the end of `syntax`"
        );
        let mut byte = 0;
        while byte < text.len() {
            assert!(
                syntax[offset + byte] == text[byte],
                "token text does not match `syntax`"
            );
            byte += 1;
        }
        lengths[index] = (kind, text.len());
        offset += text.len();
        index += 1;
    }
    assert!(
        offset == syntax.len(),
        "tokens do not cover all of `syntax`"
    );
    lengths
}

impl ProgrammingLanguageVersion {
    /// This version's tokens as `(kind, text)`, slicing [`syntax`](Self::syntax) by the
    /// recorded lengths.
    pub fn tokens(&self) -> impl Iterator<Item = (TokenKind, &'static str)> {
        let syntax = self.syntax;
        self.tokenized_syntax
            .iter()
            .scan(0, move |offset, &(kind, len)| {
                let text = &syntax[*offset..*offset + len];
                *offset += len;
                Some((kind, text))
            })
    }

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
        for (token_kind, text) in self.tokens() {
            let style = token_kind.style(resolved_theme);
            // `{style}` / `{style:#}` render the style's ANSI start / reset codes. Writing
            // to a `String` never fails, so the `fmt::Result` is safe to ignore.
            let _ = write!(collected, "{style}{text}{style:#}");
        }
        collected
    }
}
