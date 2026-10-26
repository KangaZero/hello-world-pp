//! Hello-world snippet for `ProgrammingLanguageName::Readable`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Readable>
//! - <https://esolangs.org/wiki/Readable>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Readable => "Readable",
    versions: [Readable => "Readable"],
    extensions: [],
}

const READABLE: &str = r"−−−-−-−---−−−-----−------−−-−-----−---−−−-−---−-−----−−-−---−-−----−−-−---−-−-−-−−−---−-−----−−----------−−-−-−---−-−-−−−-−---−-−-−-−−−-−-−-----−--−−-−---−-−----−−-−-----−----−−---------−−";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Readable,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Readable,
            syntax: READABLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    READABLE,
                    [(
                        TokenKind::Plain,
                        r"−−−-−-−---−−−-----−------−−-−-----−---−−−-−---−-−----−−-−---−-−----−−-−---−-−-−-−−−---−-−----−−----------−−-−-−---−-−-−−−-−---−-−-−-−−−-−-−-----−--−−-−---−-−----−−-−-----−----−−---------−−",
                    )],
                )
            },
        }],
    };
