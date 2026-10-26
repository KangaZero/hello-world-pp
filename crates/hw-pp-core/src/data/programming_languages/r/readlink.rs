//! Hello-world snippet for `ProgrammingLanguageName::Readlink`.
//!
//! Imported from leachim6/hello-world (`r/Readlink.readlink` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Readlink.readlink>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Readlink => "Readlink",
    versions: [Readlink => "Readlink"],
    extensions: [Readlink => "readlink"],
}

const READLINK: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Readlink,
        extensions: &[FileExtension::Readlink],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Readlink,
            syntax: READLINK,
            tokenized_syntax: &const {
                checked_token_lengths(READLINK, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
