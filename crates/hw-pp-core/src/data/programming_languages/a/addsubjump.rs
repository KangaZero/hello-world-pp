//! Hello-world snippet for `ProgrammingLanguageName::Addsubjump`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AddSubJump>
//! - <https://esolangs.org/wiki/AddSubJump>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Addsubjump => "AddSubJump",
    versions: [Addsubjump => "AddSubJump"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Addsubjump,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Addsubjump,
            syntax: r"IO H
1 @1
CMP 1 E
IFZ temp IO 0
.data H: 72 101 108 108 111 44 32 119 111 114 108 100 33 E:E",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"IO H
1 @1
CMP 1 E
IFZ temp IO 0
.data H: 72 101 108 108 111 44 32 119 111 114 108 100 33 E:E",
            )],
        }],
    };
