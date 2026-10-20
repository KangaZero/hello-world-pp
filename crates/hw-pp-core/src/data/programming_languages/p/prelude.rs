//! Hello-world snippet for `ProgrammingLanguageName::Prelude`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Prelude>
//! - <https://esolangs.org/wiki/Prelude>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Prelude => "Prelude",
    versions: [Prelude => "Prelude"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prelude,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Prelude,
            syntax: r"...8+.!v...4+.1+!..v!..vvv...4+.!vvv.....9+.3-!###!..v6-!..vv..v
9+(1-)^#7+(1-)..^7+!^3+!##8+(1-)^###9+1+(1-)..^^^^^3+!..^8-!#1+!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"...8+.!v...4+.1+!..v!..vvv...4+.!vvv.....9+.3-!###!..v6-!..vv..v
9+(1-)^#7+(1-)..^7+!^3+!##8+(1-)^###9+1+(1-)..^^^^^3+!..^8-!#1+!",
            )],
        }],
    };
