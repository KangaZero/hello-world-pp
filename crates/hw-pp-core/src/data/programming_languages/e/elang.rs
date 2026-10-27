//! Hello-world snippet for `ProgrammingLanguageName::Elang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#ELANG>
//! - <https://esolangs.org/wiki/ELANG>

use crate::data::programming_languages::ProgrammingParadigm::TwoDimensional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Elang => "ELANG",
    versions: [Elang => "ELANG"],
    extensions: [],
}

const ELANG: &str = r#""" .H .e .l .l .o ., ._ .w .o .r .l .d .! ()"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elang,
            syntax: ELANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELANG,
                    [(
                        TokenKind::Plain,
                        r#""" .H .e .l .l .o ., ._ .w .o .r .l .d .! ()"#,
                    )],
                )
            },
        }],
    };
