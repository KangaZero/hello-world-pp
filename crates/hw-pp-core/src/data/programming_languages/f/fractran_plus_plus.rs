//! Hello-world snippet for `ProgrammingLanguageName::FractranPlusPlus`.
//!
//! Imported from leachim6/hello-world (`f/Fractran++.fpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fractran%2B%2B.fpp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: FractranPlusPlus => "Fractran++",
    versions: [FractranPlusPlus => "Fractran++"],
    extensions: [Fpp => "fpp"],
}

const FRACTRAN_PLUS_PLUS: &str =
    r"3,-1/2,(2*37)/3,0/0,<71 101 108 108 111 32 87 111 114 108 100 0>/37,4/0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FractranPlusPlus,
        extensions: &[FileExtension::Fpp],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FractranPlusPlus,
            syntax: FRACTRAN_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FRACTRAN_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        r"3,-1/2,(2*37)/3,0/0,<71 101 108 108 111 32 87 111 114 108 100 0>/37,4/0",
                    )],
                )
            },
        }],
    };
