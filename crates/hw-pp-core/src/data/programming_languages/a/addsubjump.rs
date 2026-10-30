//! Hello-world snippet for `ProgrammingLanguageName::Addsubjump`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AddSubJump>
//! - <https://esolangs.org/wiki/AddSubJump>

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Addsubjump => "AddSubJump",
    versions: [Addsubjump => "AddSubJump"],
    extensions: [],
}

const ADDSUBJUMP: &str = r"IO H
1 @1
CMP 1 E
IFZ temp IO 0
.data H: 72 101 108 108 111 44 32 119 111 114 108 100 33 E:E";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Addsubjump,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Addsubjump,
            syntax: ADDSUBJUMP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ADDSUBJUMP,
                    [(
                        TokenKind::Plain,
                        r"IO H
1 @1
CMP 1 E
IFZ temp IO 0
.data H: 72 101 108 108 111 44 32 119 111 114 108 100 33 E:E",
                    )],
                )
            },
        }],
    };
