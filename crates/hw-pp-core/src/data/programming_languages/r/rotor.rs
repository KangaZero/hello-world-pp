//! Hello-world snippet for `ProgrammingLanguageName::Rotor`.
//!
//! Imported from leachim6/hello-world (`r/Rotor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rotor>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Rotor => "Rotor",
    versions: [Rotor => "Rotor"],
    extensions: [],
}

const ROTOR: &str = concat!(r#"""#, output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rotor,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rotor,
            syntax: ROTOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ROTOR,
                    [(TokenKind::Plain, concat!(r#"""#, output_for_hello_world!()))],
                )
            },
        }],
    };
