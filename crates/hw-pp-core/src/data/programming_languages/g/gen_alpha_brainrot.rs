//! Hello-world snippet for `ProgrammingLanguageName::GenAlphaBrainrot`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Gen_Alpha_Brainrot>
//! - <https://esolangs.org/wiki/Gen_Alpha_Brainrot>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: GenAlphaBrainrot => "Gen Alpha Brainrot",
    versions: [GenAlphaBrainrot => "Gen Alpha Brainrot"],
    extensions: [],
}

const GEN_ALPHA_BRAINROT: &str = r"sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
sigma
ohio
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
ohio
sigma
sigma
sigma
ohio
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
ohio
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
ohio
rizz
rizz
rizz
rizz
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
skibidi
skibidi
ohio";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GenAlphaBrainrot,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GenAlphaBrainrot,
            syntax: GEN_ALPHA_BRAINROT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GEN_ALPHA_BRAINROT,
                    [(
                        TokenKind::Plain,
                        r"sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
sigma
ohio
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
ohio
sigma
sigma
sigma
ohio
skibidi
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
grimaceshake
skibidi
sigma
sigma
sigma
sigma
sigma
rizz
gyatt
fanumtax
skibidi
sigma
sigma
sigma
sigma
ohio
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
gyatt
ohio
rizz
rizz
rizz
rizz
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
sigma
ohio
skibidi
skibidi
ohio",
                    )],
                )
            },
        }],
    };
