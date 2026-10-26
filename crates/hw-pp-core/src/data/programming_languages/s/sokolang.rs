//! Hello-world snippet for `ProgrammingLanguageName::Sokolang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sokolang>
//! - <https://esolangs.org/wiki/Sokolang>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sokolang => "Sokolang",
    versions: [Sokolang => "Sokolang"],
    extensions: [],
}

const SOKOLANG: &str = r" ###    
##*##
#@H*#
#####

---

h:10,13,72,101,108,108,111,44,32,119,111,114,108,100,33

---
rwu";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sokolang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sokolang,
            syntax: SOKOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SOKOLANG,
                    [(
                        TokenKind::Plain,
                        r" ###    
##*##
#@H*#
#####

---

h:10,13,72,101,108,108,111,44,32,119,111,114,108,100,33

---
rwu",
                    )],
                )
            },
        }],
    };
