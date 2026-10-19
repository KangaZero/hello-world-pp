//! Hello-world snippet for `ProgrammingLanguageName::Tofunction`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#ToFunction>
//! - <https://esolangs.org/wiki/ToFunction>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tofunction => "ToFunction",
    versions: [Tofunction => "ToFunction"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tofunction,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tofunction,
            syntax: concat!(
                r#"limit ""#,
                output_for_hello_world!(),
                r#"";
input ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"limit ""#,
                    output_for_hello_world!(),
                    r#"";
input ""#,
                    output_for_hello_world!(),
                    r#"";"#
                ),
            )],
        }],
    };
