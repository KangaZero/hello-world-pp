//! Hello-world snippet for `ProgrammingLanguageName::Domain`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Domain>
//! - <https://esolangs.org/wiki/Domain>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Domain => "Domain",
    versions: [Domain => "Domain"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Domain,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Domain,
            syntax: concat!(
                r#"1/x1=""#,
                output_for_hello_world!(),
                r#""
_
"1/x1"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"1/x1=""#,
                    output_for_hello_world!(),
                    r#""
_
"1/x1"#
                ),
            )],
        }],
    };
