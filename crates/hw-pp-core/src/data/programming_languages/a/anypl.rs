//! Hello-world snippet for `ProgrammingLanguageName::Anypl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AnyPL>
//! - <https://esolangs.org/wiki/AnyPL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Anypl => "AnyPL",
    versions: [Anypl => "AnyPL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Anypl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Anypl,
            syntax: r"HelloWor 0001 .2.. S1.. .3.. .3.. Tr.. .1.c ---- ........ [ Put the strings to the tape. ]
ld...... 0010 .<.. 1.0. .<.. .... .... .... 121= ........ [ Output.                      ]
........ 0001 .... .... .... .... .... .... -11. ........ [ End execution.               ]",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"HelloWor 0001 .2.. S1.. .3.. .3.. Tr.. .1.c ---- ........ [ Put the strings to the tape. ]
ld...... 0010 .<.. 1.0. .<.. .... .... .... 121= ........ [ Output.                      ]
........ 0001 .... .... .... .... .... .... -11. ........ [ End execution.               ]",
            )],
        }],
    };
