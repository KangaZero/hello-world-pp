//! Hello-world snippet for `ProgrammingLanguageName::Alore`.
//!
//! Imported from leachim6/hello-world (`a/Alore.alore` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Alore.alore>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Alore => "Alore",
    versions: [Alore => "Alore"],
    extensions: [Alore => "alore"],
}

const ALORE: &str = concat!(r"Print('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alore,
        extensions: &[FileExtension::Alore],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alore,
            syntax: ALORE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALORE,
                    [(
                        TokenKind::Plain,
                        concat!(r"Print('", output_for_hello_world!(), r"')"),
                    )],
                )
            },
        }],
    };
