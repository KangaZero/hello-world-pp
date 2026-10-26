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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Licorne => "Licorne",
    versions: [Licorne => "Licorne"],
    extensions: [],
}

const LICORNE: &str = concat!(
    r"hurler.$.aux licornes; $",
    output_for_hello_world!(),
    r"
s'arrêter discrétos"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Licorne,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Licorne,
            syntax: LICORNE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LICORNE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"hurler.$.aux licornes; $",
                            output_for_hello_world!(),
                            r"
s'arrêter discrétos"
                        ),
                    )],
                )
            },
        }],
    };
