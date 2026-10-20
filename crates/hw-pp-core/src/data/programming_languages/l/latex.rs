//! Hello-world snippets for `ProgrammingLanguageName::Latex`.
//!
//! Sources:
//! - <https://www.latex-project.org/help/documentation/usrguide.pdf>
//! - <https://www.learnlatex.org/en/lesson-01>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Latex => "LaTeX",
    versions: [Latex2e => "LaTeX2e"],
    extensions: [Tex => "tex"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latex,
        extensions: &[FileExtension::Tex],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Latex2e,
            syntax: concat!(
                r"\documentclass{article}
\begin{document}
",
                output_for_hello_world!(),
                r"
\end{document}"
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "documentclass"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "article"),
                (TokenKind::Punctuation, "}"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "begin"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "document"),
                (TokenKind::Punctuation, "}"),
                (
                    TokenKind::Plain,
                    concat!("\n", output_for_hello_world!(), "\n"),
                ),
                (TokenKind::Punctuation, "\\"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "document"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
