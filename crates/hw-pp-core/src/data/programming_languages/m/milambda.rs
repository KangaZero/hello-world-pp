//! Hello-world snippet for `ProgrammingLanguageName::Milambda`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MiLambda>
//! - <https://esolangs.org/wiki/MiLambda>

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
    language: Milambda => "MiLambda",
    versions: [Milambda => "MiLambda"],
    extensions: [],
}

const MILAMBDA: &str = r">  EEEEEEEΔΔΘς       v
v  ςΘΔEEEEEEEEEE     <
>  EEEEEEEEEEEδδΘς   v
v  ΘδδEEEEEEEEEEEς   <
>  ς EEEEEEEEEEEΔΘ   v
v  ΘΔΔΔΔEEEEς        <
>  ςEEEΔΔΘ           v
v  ΘδEEEEEEEEEEEEς   <
>  ςEEEEEEEEEEEΔΘ    v
v  ΘΔΔΔΔEEEEEEEEEEEς <
>  ςEEEEEEEEEEEδδΘ   v
v  ΘEEEEEEEEEEς      <
>  ςEEEΔΔΔΘ          λ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Milambda,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Milambda,
            syntax: MILAMBDA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MILAMBDA,
                    [(
                        TokenKind::Plain,
                        r">  EEEEEEEΔΔΘς       v
v  ςΘΔEEEEEEEEEE     <
>  EEEEEEEEEEEδδΘς   v
v  ΘδδEEEEEEEEEEEς   <
>  ς EEEEEEEEEEEΔΘ   v
v  ΘΔΔΔΔEEEEς        <
>  ςEEEΔΔΘ           v
v  ΘδEEEEEEEEEEEEς   <
>  ςEEEEEEEEEEEΔΘ    v
v  ΘΔΔΔΔEEEEEEEEEEEς <
>  ςEEEEEEEEEEEδδΘ   v
v  ΘEEEEEEEEEEς      <
>  ςEEEΔΔΔΘ          λ",
                    )],
                )
            },
        }],
    };
