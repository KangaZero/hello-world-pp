//! Hello-world snippet for `ProgrammingLanguageName::ApplesoftBasic`.
//!
//! Imported from leachim6/hello-world (`a/Applesoft BASIC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Applesoft%20BASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: ApplesoftBasic => "Applesoft BASIC",
    versions: [ApplesoftBasic => "Applesoft BASIC"],
    extensions: [],
}

const APPLESOFT_BASIC: &str = concat!(r#"10 PRINT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ApplesoftBasic,
        extensions: &[],
        categories: &[Historical],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ApplesoftBasic,
            syntax: APPLESOFT_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    APPLESOFT_BASIC,
                    [
                        (TokenKind::Number, r"10"),
                        (TokenKind::Plain, r" "),
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
