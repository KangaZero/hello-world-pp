//! Hello-world snippet for `ProgrammingLanguageName::Divzeros`.
//!
//! Imported from leachim6/hello-world (`d/Divzeros.dz` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Divzeros.dz>
//! - <https://esolangs.org/wiki/Divzeros> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Divzeros => "Divzeros",
    versions: [Divzeros => "Divzeros"],
    extensions: [Dz => "dz"],
}

const DIVZEROS: &str = r"(?'H+?'e+?'l+?'l+?'o+?',+?' +?'W+?'o+?'r+?'l+?'d+?'!)/0";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Divzeros,
        extensions: &[FileExtension::Dz],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Divzeros,
            syntax: DIVZEROS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DIVZEROS,
                    [(
                        TokenKind::Plain,
                        r"(?'H+?'e+?'l+?'l+?'o+?',+?' +?'W+?'o+?'r+?'l+?'d+?'!)/0",
                    )],
                )
            },
        }],
    };
