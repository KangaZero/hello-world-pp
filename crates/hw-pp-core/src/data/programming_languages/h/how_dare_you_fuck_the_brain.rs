//! Hello-world snippet for `ProgrammingLanguageName::HowDareYouFuckTheBrain`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#How_dare_you_fuck_the_brain>
//! - <https://esolangs.org/wiki/How_dare_you_fuck_the_brain>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: HowDareYouFuckTheBrain => "How dare you fuck the brain",
    versions: [HowDareYouFuckTheBrain => "How dare you fuck the brain"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HowDareYouFuckTheBrain,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HowDareYouFuckTheBrain,
            syntax: r"IIII=+=+=+=+IIIIIIIIP=DDDP^IIIIPPIIIP^IIII=+=+=+^IIII=+=+DDDDv+P
DvD^)IIII=+=+=+P=+^IIII=+=+v+IIIIIIIP=
vD^D)^DDDDDDDDv+P=IIIP^DDDP^DDDD=+v+P^IIII=+=+=+IPH",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"IIII=+=+=+=+IIIIIIIIP=DDDP^IIIIPPIIIP^IIII=+=+=+^IIII=+=+DDDDv+P
DvD^)IIII=+=+=+P=+^IIII=+=+v+IIIIIIIP=
vD^D)^DDDDDDDDv+P=IIIP^DDDP^DDDD=+v+P^IIII=+=+=+IPH",
            )],
        }],
    };
