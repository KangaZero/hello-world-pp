//! Hello-world snippet for `ProgrammingLanguageName::Retina`.
//!
//! Imported from leachim6/hello-world (`r/Retina.ret` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Retina.ret>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Retina => "Retina",
    versions: [Retina => "Retina"],
    extensions: [Ret => "ret"],
}

const RETINA: &str = concat!(r"K`", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Retina,
        extensions: &[FileExtension::Ret],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Retina,
            syntax: RETINA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RETINA,
                    [(TokenKind::Plain, concat!(r"K`", output_for_hello_world!()))],
                )
            },
        }],
    };
