//! Hello-world snippets for `ProgrammingLanguageName::Css`.
//!
//! Sources:
//! - <https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::before>
//! - <https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/content>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Css => "CSS",
    versions: [Css3 => "CSS3"],
    extensions: [Css => "css"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Css,
        extensions: &[FileExtension::Css],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Css3,
            syntax: concat!(
                r#"body::before {
    content: ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "body"),
                (TokenKind::Punctuation, "::"),
                (TokenKind::Plain, "before "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Type, "content"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
