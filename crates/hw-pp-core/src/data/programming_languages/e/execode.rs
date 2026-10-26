//! Hello-world snippet for `ProgrammingLanguageName::Execode`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Execode>
//! - <https://esolangs.org/wiki/Execode>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Execode => "Execode",
    versions: [Execode => "Execode"],
    extensions: [],
}

const EXECODE: &str = r"def var 0
inc 0 rpt 72
outchr 0
inc 0 rpt 29
outchr 0
inc 0 rpt 7
outchr 0 rpt 2
inc 0 rpt 3
outchr 0
dec 0 rpt 67
outchr 0
dec 0 rpt 12
outchr 0
inc 0 rpt 55
outchr 0
inc 0 rpt 24
outchr 0
inc 0 rpt 3
outchr 0
dec 0 rpt 6
outchr 0
dec 0 rpt 8
outchr 0
dec 0 rpt 67
outchr 0
ter";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Execode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Execode,
            syntax: EXECODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EXECODE,
                    [(
                        TokenKind::Plain,
                        r"def var 0
inc 0 rpt 72
outchr 0
inc 0 rpt 29
outchr 0
inc 0 rpt 7
outchr 0 rpt 2
inc 0 rpt 3
outchr 0
dec 0 rpt 67
outchr 0
dec 0 rpt 12
outchr 0
inc 0 rpt 55
outchr 0
inc 0 rpt 24
outchr 0
inc 0 rpt 3
outchr 0
dec 0 rpt 6
outchr 0
dec 0 rpt 8
outchr 0
dec 0 rpt 67
outchr 0
ter",
                    )],
                )
            },
        }],
    };
