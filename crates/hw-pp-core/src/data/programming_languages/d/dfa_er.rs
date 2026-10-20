//! Hello-world snippet for `ProgrammingLanguageName::DfaEr`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DFA-er>
//! - <https://esolangs.org/wiki/DFA-er>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: DfaEr => "DFA-er",
    versions: [DfaEr => "DFA-er"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DfaEr,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DfaEr,
            syntax: r".1001000.
--1100101-
.1100101.
--1101100-
.1101100.
--1101100-
-1-1101111-
-10-1100100-
.1101111.
--101100-
-1-1110010-
.101100.
--100000-
.100000.
--1110111-
.1110111.
--1101111-
.1110010.
--1101100-
.1100100.
--100001-
..100001.
!
.......1..........1....10...",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".1001000.
--1100101-
.1100101.
--1101100-
.1101100.
--1101100-
-1-1101111-
-10-1100100-
.1101111.
--101100-
-1-1110010-
.101100.
--100000-
.100000.
--1110111-
.1110111.
--1101111-
.1110010.
--1101100-
.1100100.
--100001-
..100001.
!
.......1..........1....10...",
            )],
        }],
    };
