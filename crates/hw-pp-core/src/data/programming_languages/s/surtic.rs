//! Hello-world snippet for `ProgrammingLanguageName::Surtic`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Surtic>
//! - <https://esolangs.org/wiki/Surtic>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Surtic => "Surtic",
    versions: [Surtic => "Surtic"],
    extensions: [],
}

const SURTIC: &str = r"c0++++++++fc0[c1++++fc1[c1‑c2++c3+++c4+++c5+]c0‑c2+c3+c4‑c6+]oc2c3---oc3c3+++++++oc3oc3c3+++oc3oc5c4‑oc4oc3c3+++oc3c3------oc3fc6[c3-]oc3c5+oc5c6++oc6";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Surtic,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Surtic,
            syntax: SURTIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SURTIC,
                    [(
                        TokenKind::Plain,
                        r"c0++++++++fc0[c1++++fc1[c1‑c2++c3+++c4+++c5+]c0‑c2+c3+c4‑c6+]oc2c3---oc3c3+++++++oc3oc3c3+++oc3oc5c4‑oc4oc3c3+++oc3c3------oc3fc6[c3-]oc3c5+oc5c6++oc6",
                    )],
                )
            },
        }],
    };
