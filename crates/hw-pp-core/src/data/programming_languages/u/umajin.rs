//! Hello-world snippet for `ProgrammingLanguageName::Umajin`.
//!
//! Imported from leachim6/hello-world (`u/Umajin.u` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Umajin.u>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Umajin => "Umajin",
    versions: [Umajin => "Umajin"],
    extensions: [U => "u"],
}

const UMAJIN: &str = concat!(
    r"println '",
    output_for_hello_world!(),
    r"'
"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Umajin,
        extensions: &[FileExtension::U],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Umajin,
            syntax: UMAJIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UMAJIN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"println '",
                            output_for_hello_world!(),
                            r"'
"
                        ),
                    )],
                )
            },
        }],
    };
