//! Hello-world snippet for `ProgrammingLanguageName::Brainbits`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brainbits>
//! - <https://esolangs.org/wiki/Brainbits>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Brainbits => "Brainbits",
    versions: [Brainbits => "Brainbits"],
    extensions: [],
}

const BRAINBITS: &str = r">\>>>\>>>><<<<<<<<.>\>>>\>>>><<<<<<<<>\>\>>>\>>\><<<<<<<<.>\>\>>>\>>\><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>\>\>\>\><<<<<<<<.>\>\>>\>\>\>\><<<<<<<<>>\>>>>>><<<<<<<<.>>\>>>>>><<<<<<<<>\>>\>>\>\>\><<<<<<<<.>\>>\>>\>\>\><<<<<<<<>\>\>>\>\>\>\><<<<<<<<.>\>\>>\>\>\>\><<<<<<<<>\>\>\>>>\>><<<<<<<<.>\>\>\>>>\>><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>>\>>><<<<<<<<.>\>\>>>\>>><<<<<<<<>>\>>>>>\><<<<<<<<.>>\>>>>>\><<<<<<<<";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainbits,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainbits,
            syntax: BRAINBITS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINBITS,
                    [(
                        TokenKind::Plain,
                        r">\>>>\>>>><<<<<<<<.>\>>>\>>>><<<<<<<<>\>\>>>\>>\><<<<<<<<.>\>\>>>\>>\><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>\>\>\>\><<<<<<<<.>\>\>>\>\>\>\><<<<<<<<>>\>>>>>><<<<<<<<.>>\>>>>>><<<<<<<<>\>>\>>\>\>\><<<<<<<<.>\>>\>>\>\>\><<<<<<<<>\>\>>\>\>\>\><<<<<<<<.>\>\>>\>\>\>\><<<<<<<<>\>\>\>>>\>><<<<<<<<.>\>\>\>>>\>><<<<<<<<>\>\>>\>\>>><<<<<<<<.>\>\>>\>\>>><<<<<<<<>\>\>>>\>>><<<<<<<<.>\>\>>>\>>><<<<<<<<>>\>>>>>\><<<<<<<<.>>\>>>>>\><<<<<<<<",
                    )],
                )
            },
        }],
    };
