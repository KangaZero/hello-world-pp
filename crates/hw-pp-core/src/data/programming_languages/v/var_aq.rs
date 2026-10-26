//! Hello-world snippet for `ProgrammingLanguageName::VarAq`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#var'aq>
//! - <https://esolangs.org/wiki/Var'aq>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VarAq => "var'aq",
    versions: [VarAq => "var'aq"],
    extensions: [],
}

const VAR_AQ: &str = concat!(r#"""#, output_for_hello_world!(), r#"" cha'"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VarAq,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VarAq,
            syntax: VAR_AQ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VAR_AQ,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" cha'"#),
                    )],
                )
            },
        }],
    };
