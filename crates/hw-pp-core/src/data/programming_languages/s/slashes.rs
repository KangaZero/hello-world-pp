//! Hello-world snippet for `ProgrammingLanguageName::Slashes`.
//!
//! Imported from leachim6/hello-world (`s/Slashes.slashes` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Slashes.slashes>
//! - <https://esolangs.org/wiki/%2F%2F%2F> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, StringRewriting};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Slashes => "Slashes",
    versions: [Slashes => "Slashes"],
    extensions: [Slashes => "slashes"],
}

const SLASHES: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Slashes,
        extensions: &[FileExtension::Slashes],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Slashes,
            syntax: SLASHES,
            tokenized_syntax: &const {
                checked_token_lengths(SLASHES, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
