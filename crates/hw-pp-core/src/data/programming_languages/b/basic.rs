//! Hello-world snippets for `ProgrammingLanguageName::Basic`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Dartmouth_BASIC>
//! - <https://en.wikipedia.org/wiki/BASIC>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Basic => "BASIC",
    versions: [ClassicBasic => "Classic BASIC"],
    extensions: [Bas => "bas"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Basic,
        extensions: &[FileExtension::Bas],
        categories: &[Scripting, Historical],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClassicBasic,
            syntax: concat!(
                r#"10 PRINT ""#,
                output_for_hello_world!(),
                r#""
20 END"#
            ),
            tokenized_syntax: &[
                (TokenKind::Number, "10"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "PRINT"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Number, "20"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "END"),
            ],
        }],
    };
