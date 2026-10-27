//! Hello-world snippet for `ProgrammingLanguageName::Omgrofl`.
//!
//! Imported from leachim6/hello-world (`o/Omgrofl.omgrofl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Omgrofl.omgrofl>
//! - <https://esolangs.org/wiki/Omgrofl> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{QueueBased, StackBased};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Omgrofl => "Omgrofl",
    versions: [Omgrofl => "Omgrofl"],
    extensions: [Omgrofl => "omgrofl"],
}

const OMGROFL: &str = r"lol iz 72
rofl lol
lol iz 101
rofl lol
lol iz 108
rofl lol
rofl lol
lool iz 111
rofl lool
loool iz 44
rofl loool
loool iz 32
rofl loool
loool iz 87
rofl loool
rofl lool
lool iz 114
rofl lool
rofl lol
lol iz 100
rofl lol
lol iz 33
rofl lol";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Omgrofl,
        extensions: &[FileExtension::Omgrofl],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Omgrofl,
            syntax: OMGROFL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OMGROFL,
                    [(
                        TokenKind::Plain,
                        r"lol iz 72
rofl lol
lol iz 101
rofl lol
lol iz 108
rofl lol
rofl lol
lool iz 111
rofl lool
loool iz 44
rofl loool
loool iz 32
rofl loool
loool iz 87
rofl loool
rofl lool
lool iz 114
rofl lool
rofl lol
lol iz 100
rofl lol
lol iz 33
rofl lol",
                    )],
                )
            },
        }],
    };
