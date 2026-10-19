//! Hello-world snippet for `ProgrammingLanguageName::Aardvark`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Aardvark>
//! - <https://esolangs.org/wiki/Aardvark>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Aardvark => "Aardvark",
    versions: [Aardvark => "Aardvark"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aardvark,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aardvark,
            syntax: concat!(
                r#"async main() {
    println ""#,
                output_for_hello_world!(),
                r#""
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"async main() {
    println ""#,
                    output_for_hello_world!(),
                    r#""
}"#
                ),
            )],
        }],
    };
