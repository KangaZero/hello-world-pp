//! Hello-world snippet for `ProgrammingLanguageName::Gecho`.
//!
//! Imported from leachim6/hello-world (`g/Gecho.gecho` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gecho.gecho>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gecho => "Gecho",
    versions: [Gecho => "Gecho"],
    extensions: [Gecho => "gecho"],
}

const GECHO: &str = r"print 'hello '<> 'world";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gecho,
        extensions: &[FileExtension::Gecho],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gecho,
            syntax: GECHO,
            tokenized_syntax: &const {
                checked_token_lengths(GECHO, [(TokenKind::Plain, r"print 'hello '<> 'world")])
            },
        }],
    };
