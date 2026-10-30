//! Hello-world snippet for `ProgrammingLanguageName::Objlang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ObjLang>
//! - <https://esolangs.org/wiki/ObjLang>

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Objlang => "ObjLang",
    versions: [Objlang => "ObjLang"],
    extensions: [],
}

const OBJLANG: &str = r"e9oFcHJpbnRbAQAAAPoNSGVsbG8sIFdvcmxkITA=";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Objlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Objlang,
            syntax: OBJLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBJLANG,
                    [(
                        TokenKind::Plain,
                        r"e9oFcHJpbnRbAQAAAPoNSGVsbG8sIFdvcmxkITA=",
                    )],
                )
            },
        }],
    };
