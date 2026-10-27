//! Hello-world snippet for `ProgrammingLanguageName::Sansism`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sansism>
//! - <https://esolangs.org/wiki/Sansism>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sansism => "Sansism",
    versions: [Sansism => "Sansism"],
    extensions: [],
}

const SANSISM: &str = r"           G                                  GGG
>++++++++++>!+++++++!++++++++++!+++!+##!!!!##-G+G
G.+++++++++++++++##!!##.++!.+++..+++++++.+!.++! G
G!.+++.------.--------.!+.!.G                  GG";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sansism,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sansism,
            syntax: SANSISM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SANSISM,
                    [(
                        TokenKind::Plain,
                        r"           G                                  GGG
>++++++++++>!+++++++!++++++++++!+++!+##!!!!##-G+G
G.+++++++++++++++##!!##.++!.+++..+++++++.+!.++! G
G!.+++.------.--------.!+.!.G                  GG",
                    )],
                )
            },
        }],
    };
