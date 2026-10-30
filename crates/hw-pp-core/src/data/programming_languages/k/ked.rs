//! Hello-world snippet for `ProgrammingLanguageName::Ked`.
//!
//! Imported from leachim6/hello-world (`k/Ked.ked` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Ked.ked>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ked => "Ked",
    versions: [Ked => "Ked"],
    extensions: [Ked => "ked"],
}

const KED: &str = concat!(r"saysI '", output_for_hello_world!(), r"' like");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ked,
        extensions: &[FileExtension::Ked],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ked,
            syntax: KED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KED,
                    [(
                        TokenKind::Plain,
                        concat!(r"saysI '", output_for_hello_world!(), r"' like"),
                    )],
                )
            },
        }],
    };
