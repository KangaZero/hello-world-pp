//! Hello-world snippet for `ProgrammingLanguageName::Inkling`.
//!
//! Imported from leachim6/hello-world (`i/Inkling.ink` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Inkling.ink>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Inkling => "Inkling",
    versions: [Inkling => "Inkling"],
    extensions: [],
}

const INKLING: &str = concat!(r#"display ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Inkling,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Inkling,
            syntax: INKLING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    INKLING,
                    [(
                        TokenKind::Plain,
                        concat!(r#"display ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
