//! Hello-world snippets for `ProgrammingLanguageName::Xml`.
//!
//! Sources:
//! - <https://www.w3.org/TR/xml/>
//! - <https://developer.mozilla.org/en-US/docs/Web/XML/Guides/XML_introduction>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xml => "XML",
    versions: [Xml10 => "XML 1.0"],
    extensions: [Xml => "xml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xml,
        extensions: &[FileExtension::Xml],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xml10,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<greeting>"#,
                output_for_hello_world!(),
                r#"</greeting>"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<?"),
                (TokenKind::Plain, "xml version"),
                (TokenKind::Punctuation, "="),
                (TokenKind::String, "\"1.0\""),
                (TokenKind::Plain, " encoding"),
                (TokenKind::Punctuation, "="),
                (TokenKind::String, "\"UTF-8\""),
                (TokenKind::Punctuation, "?>"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "<"),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Punctuation, "</"),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ">"),
            ],
        }],
    };
