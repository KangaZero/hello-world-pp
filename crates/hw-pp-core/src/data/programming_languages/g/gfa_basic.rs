//! Hello-world snippet for `ProgrammingLanguageName::GfaBasic`.
//!
//! Imported from leachim6/hello-world (`g/GFA Basic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GFA%20Basic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GfaBasic => "GFA Basic",
    versions: [GfaBasic => "GFA Basic"],
    extensions: [],
}

const GFA_BASIC: &str = concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GfaBasic,
        extensions: &[],
        categories: &[Historical],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GfaBasic,
            syntax: GFA_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GFA_BASIC,
                    [
                        (TokenKind::Keyword, r"PRINT"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
