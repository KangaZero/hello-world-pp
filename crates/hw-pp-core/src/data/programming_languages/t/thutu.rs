//! Hello-world snippet for `ProgrammingLanguageName::Thutu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Thutu>
//! - <https://esolangs.org/wiki/Thutu>

use crate::data::programming_languages::ProgrammingParadigm::StringRewriting;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Thutu => "Thutu",
    versions: [Thutu => "Thutu"],
    extensions: [],
}

const THUTU: &str = concat!(r"/^=1$/", output_for_hello_world!(), r"=n=x=9/");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thutu,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thutu,
            syntax: THUTU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    THUTU,
                    [(
                        TokenKind::Plain,
                        concat!(r"/^=1$/", output_for_hello_world!(), r"=n=x=9/"),
                    )],
                )
            },
        }],
    };
