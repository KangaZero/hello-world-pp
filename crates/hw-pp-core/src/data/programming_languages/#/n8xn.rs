//! Hello-world snippet for `ProgrammingLanguageName::N8xn`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#8xn>
//! - <https://esolangs.org/wiki/8xn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: N8xn => "8xn",
    versions: [N8xn => "8xn"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N8xn,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N8xn,
            syntax: r"8x1111111131111111119>6311311111831111111111433393111111111191>611111116611164333388311111111111311119>63311111111311119>6433333331192>64
33336111622222262222222263111111113111191>6",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"8x1111111131111111119>6311311111831111111111433393111111111191>611111116611164333388311111111111311119>63311111111311119>6433333331192>64
33336111622222262222222263111111113111191>6",
            )],
        }],
    };
