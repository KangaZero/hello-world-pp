//! Hello-world snippet for `ProgrammingLanguageName::N420`.
//!
//! Imported from leachim6/hello-world (`#/420.420` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/420.420>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N420 => "420",
    versions: [N420 => "420"],
    extensions: [E420 => "420"],
}

const N420: &str = r"420
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it
blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N420,
        extensions: &[FileExtension::E420],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N420,
            syntax: N420,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N420,
                    [(
                        TokenKind::Plain,
                        r"420
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it
blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it",
                    )],
                )
            },
        }],
    };
