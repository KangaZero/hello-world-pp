//! Hello-world snippet for `ProgrammingLanguageName::Highfive`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#HighFive>
//! - <https://esolangs.org/wiki/HighFive>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Highfive => "HighFive",
    versions: [Highfive => "HighFive"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Highfive,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Highfive,
            syntax: r"-----------------------------------////++++++++////
/+++++++++/+++++++++++++/++++/-////*/
./---.+++++++..+++././/////+++++++++++++++./.+++.------.--------./+.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"-----------------------------------////++++++++////
/+++++++++/+++++++++++++/++++/-////*/
./---.+++++++..+++././/////+++++++++++++++./.+++.------.--------./+.",
            )],
        }],
    };
