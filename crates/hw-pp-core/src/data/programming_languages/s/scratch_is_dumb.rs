//! Hello-world snippet for `ProgrammingLanguageName::ScratchIsDumb`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Scratch_is_dumb>
//! - <https://esolangs.org/wiki/Scratch_is_dumb>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ScratchIsDumb => "Scratch is dumb",
    versions: [ScratchIsDumb => "Scratch is dumb"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ScratchIsDumb,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ScratchIsDumb,
            syntax: r"00 00 00 00 00 00 00 00 06 05 
00 00 00 00 06 05 00 00 05 00 
00 00 05 00 00 00 05 00 04 04 
04 04 01 07 05 00 05 00 05 01 
05 05 00 06 04 07 04 01 07 05 
05 03 05 01 01 01 03 00 00 00 
00 00 00 00 03 03 00 00 00 03 
05 05 03 04 01 03 04 03 00 00 
00 03 01 01 01 01 01 01 03 01 
01 01 01 01 01 01 01 03 05 05 
00 03 05 00 00 03",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"00 00 00 00 00 00 00 00 06 05 
00 00 00 00 06 05 00 00 05 00 
00 00 05 00 00 00 05 00 04 04 
04 04 01 07 05 00 05 00 05 01 
05 05 00 06 04 07 04 01 07 05 
05 03 05 01 01 01 03 00 00 00 
00 00 00 00 03 03 00 00 00 03 
05 05 03 04 01 03 04 03 00 00 
00 03 01 01 01 01 01 01 03 01 
01 01 01 01 01 01 01 03 05 05 
00 03 05 00 00 03",
            )],
        }],
    };
