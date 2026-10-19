//! Hello-world snippet for `ProgrammingLanguageName::U2615Yappacino`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#%E2%98%95Yappacino>
//! - <https://esolangs.org/wiki/Yappacino>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: U2615Yappacino => "☕Yappacino",
    versions: [U2615Yappacino => "☕Yappacino"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U2615Yappacino,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U2615Yappacino,
            syntax: concat!(
                r#"C:\Standard\System\io\format\print\ln(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"C:\Standard\System\io\format\print\ln(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };
