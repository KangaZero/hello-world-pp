//! Hello-world snippet for `ProgrammingLanguageName::Ink`.
//!
//! Imported from leachim6/hello-world (`i/Ink.ink` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Ink.ink>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ink => "Ink",
    versions: [Ink => "Ink"],
    extensions: [Ink => "ink"],
}

const INK: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ink,
        extensions: &[FileExtension::Ink],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ink,
            syntax: INK,
            tokenized_syntax: &const {
                checked_token_lengths(INK, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
