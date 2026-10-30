//! Hello-world snippet for `ProgrammingLanguageName::N420`.
//!
//! Imported from leachim6/hello-world (`#/420.420` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/420.420>
//! - <https://esolangs.org/wiki/420> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

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
        paradigms: &[StackBased],
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
