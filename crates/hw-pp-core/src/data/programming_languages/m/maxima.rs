//! Hello-world snippet for `ProgrammingLanguageName::Maxima`.
//!
//! Imported from leachim6/hello-world (`m/Maxima.max` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Maxima.max>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Maxima => "Maxima",
    versions: [Maxima => "Maxima"],
    extensions: [Max => "max"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maxima,
        extensions: &[FileExtension::Max],
        categories: &[Scientific],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maxima,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")$"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r"$"),
            ],
        }],
    };
