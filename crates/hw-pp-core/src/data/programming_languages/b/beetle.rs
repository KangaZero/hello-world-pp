//! Hello-world snippet for `ProgrammingLanguageName::Beetle`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Beetle>
//! - <https://esolangs.org/wiki/Beetle>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Beetle => "Beetle",
    versions: [Beetle => "Beetle"],
    extensions: [],
}

const BEETLE: &str = r"{><}/{><}></{><}/{><}/{><}></{><}/{><}/{><}/|{><}/{><}></{><}></{><}/{><}/{><}></{><}/{><}></|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}></{> 
<}></{><}></{><}></|{><}/{><}></{><}/{><}></{><}></{><}/{><}/|{><}></{><}/{><}/{><}/{><}/{><}/|{><}/{><}></{><}/{><}></{><}/{><}></{><}></{><}></|{><}/{><}></{><}></{><}/{><}></{><}></{><}></{><}></|{><}/{><}></{><}></{><}> 
</{><}/{><}/{><}></{><}/|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}/{><}></{><}/{><}/|{><}/{><}></{><}/{><}/{><}/{><}/{><}></|";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beetle,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beetle,
            syntax: BEETLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BEETLE,
                    [(
                        TokenKind::Plain,
                        r"{><}/{><}></{><}/{><}/{><}></{><}/{><}/{><}/|{><}/{><}></{><}></{><}/{><}/{><}></{><}/{><}></|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}></{> 
<}></{><}></{><}></|{><}/{><}></{><}/{><}></{><}></{><}/{><}/|{><}></{><}/{><}/{><}/{><}/{><}/|{><}/{><}></{><}/{><}></{><}/{><}></{><}></{><}></|{><}/{><}></{><}></{><}/{><}></{><}></{><}></{><}></|{><}/{><}></{><}></{><}> 
</{><}/{><}/{><}></{><}/|{><}/{><}></{><}></{><}/{><}></{><}></{><}/{><}/|{><}/{><}></{><}></{><}/{><}/{><}></{><}/{><}/|{><}/{><}></{><}/{><}/{><}/{><}/{><}></|",
                    )],
                )
            },
        }],
    };
