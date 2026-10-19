//! Hello-world snippet for `ProgrammingLanguageName::FlashShockwaveHasBeenDiscontinued`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Flash_shockwave_has_been_discontinued.>
//! - <https://esolangs.org/wiki/Flash_shockwave_has_been_discontinued.>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: FlashShockwaveHasBeenDiscontinued => "Flash shockwave has been discontinued.",
    versions: [FlashShockwaveHasBeenDiscontinued => "Flash shockwave has been discontinued."],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FlashShockwaveHasBeenDiscontinued,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FlashShockwaveHasBeenDiscontinued,
            syntax: r"aueI90ds¢£³m%#oiDijij@¨!&#&#IO21(()=§§ªD@@",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"aueI90ds¢£³m%#oiDijij@¨!&#&#IO21(()=§§ªD@@",
            )],
        }],
    };
