//! Hello-world snippet for `ProgrammingLanguageName::Traxler`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Traxler>
//! - <https://esolangs.org/wiki/Traxler>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Traxler => "Traxler",
    versions: [Traxler => "Traxler"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Traxler,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Traxler,
            syntax: concat!(
                r#"/---------------------\
|  /""#,
                output_for_hello_world!(),
                r#""\   |
|  |              |   |
|  |              |   |
|  B              _   |
\---------------------/"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"/---------------------\
|  /""#,
                    output_for_hello_world!(),
                    r#""\   |
|  |              |   |
|  |              |   |
|  B              _   |
\---------------------/"#
                ),
            )],
        }],
    };
