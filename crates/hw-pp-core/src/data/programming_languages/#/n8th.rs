//! Hello-world snippet for `ProgrammingLanguageName::N8th`.
//!
//! Imported from leachim6/hello-world (`#/8th` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/8th>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N8th => "8th",
    versions: [N8th => "8th"],
    extensions: [],
}

const N8TH: &str = concat!(r#"""#, output_for_hello_world!(), r#"\n" ."#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N8th,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N8th,
            syntax: N8TH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N8TH,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"\n" ."#),
                    )],
                )
            },
        }],
    };
