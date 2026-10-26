//! Hello-world snippet for `ProgrammingLanguageName::E62qpodb593`.
//!
//! Imported from leachim6/hello-world (`e/e62qpodb593` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/e62qpodb593>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: E62qpodb593 => "e62qpodb593",
    versions: [E62qpodb593 => "e62qpodb593"],
    extensions: [],
}

const E62QPODB593: &str = r"tqqqyoyyyyyypoyqqooqqqoztkkkkkkppotyyyyyppo2oqqqokpokkqqo";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::E62qpodb593,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::E62qpodb593,
            syntax: E62QPODB593,
            tokenized_syntax: &const {
                checked_token_lengths(
                    E62QPODB593,
                    [(
                        TokenKind::Plain,
                        r"tqqqyoyyyyyypoyqqooqqqoztkkkkkkppotyyyyyppo2oqqqokpokkqqo",
                    )],
                )
            },
        }],
    };
