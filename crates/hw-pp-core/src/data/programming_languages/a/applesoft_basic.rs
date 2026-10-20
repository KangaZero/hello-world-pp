//! Hello-world snippet for `ProgrammingLanguageName::ApplesoftBasic`.
//!
//! Imported from leachim6/hello-world (`a/Applesoft BASIC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Applesoft%20BASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ApplesoftBasic => "Applesoft BASIC",
    versions: [ApplesoftBasic => "Applesoft BASIC"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ApplesoftBasic,
        extensions: &[],
        categories: &[Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ApplesoftBasic,
            syntax: concat!(r#"10 PRINT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Number, r"10"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"PRINT"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
