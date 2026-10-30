//! Hello-world snippet for `ProgrammingLanguageName::Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeebasic`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#WeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeBasic>
//! - <https://esolangs.org/wiki/WeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeebasic => "WeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeBasic",
    versions: [Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeebasic => "WeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeBasic"],
    extensions: [],
}

const WEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEBASIC: &str =
    concat!(r#"Print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeebasic,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Weeeeeeeeeeeeeeeeeeeeeeeeeeeeeebasic,
            syntax: WEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEBASIC,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
