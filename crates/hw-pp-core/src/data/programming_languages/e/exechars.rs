//! Hello-world snippet for `ProgrammingLanguageName::Exechars`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Exechars>
//! - <https://esolangs.org/wiki/Exechars>

use crate::data::programming_languages::ProgrammingParadigm::{Functional, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Exechars => "Exechars",
    versions: [Exechars => "Exechars"],
    extensions: [],
}

const EXECHARS: &str =
    r"r48+0o0r65+1o1r6c+2o2o2r6f+3o3r2c+4o4r20+5o5r57+6o6o3r72+7o7o2r64+8o8+5o5t";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Exechars,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Exechars,
            syntax: EXECHARS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EXECHARS,
                    [(
                        TokenKind::Plain,
                        r"r48+0o0r65+1o1r6c+2o2o2r6f+3o3r2c+4o4r20+5o5r57+6o6o3r72+7o7o2r64+8o8+5o5t",
                    )],
                )
            },
        }],
    };
