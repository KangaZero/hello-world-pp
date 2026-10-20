//! Hello-world snippet for `ProgrammingLanguageName::Minusintplus`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MinusIntPlus>
//! - <https://esolangs.org/wiki/MinusOrPlus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Minusintplus => "MinusIntPlus",
    versions: [Minusintplus => "MinusIntPlus"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Minusintplus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Minusintplus,
            syntax: r"-184+-228+-250+-+-254+-68+-13+-202+-233+-254+-7+-9+-68+",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"-184+-228+-250+-+-254+-68+-13+-202+-233+-254+-7+-9+-68+",
            )],
        }],
    };
