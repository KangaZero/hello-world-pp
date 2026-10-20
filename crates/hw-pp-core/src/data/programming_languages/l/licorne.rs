//! Hello-world snippet for `ProgrammingLanguageName::Licorne`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Licorne>
//! - <https://esolangs.org/wiki/Licorne>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Licorne => "Licorne",
    versions: [Licorne => "Licorne"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Licorne,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Licorne,
            syntax: concat!(
                r"hurler.$.aux licornes; $",
                output_for_hello_world!(),
                r"
s'arrêter discrétos"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"hurler.$.aux licornes; $",
                    output_for_hello_world!(),
                    r"
s'arrêter discrétos"
                ),
            )],
        }],
    };
