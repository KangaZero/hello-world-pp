//! Hello-world snippet for `ProgrammingLanguageName::Cufrab`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Cufrab>
//! - <https://esolangs.org/wiki/Cufrab>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cufrab => "Cufrab",
    versions: [Cufrab => "Cufrab"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cufrab,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cufrab,
            syntax: concat!(
                r#":PRINT[-]~\.;
""#,
                output_for_hello_world!(),
                r#"\n"PRINT"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#":PRINT[-]~\.;
""#,
                    output_for_hello_world!(),
                    r#"\n"PRINT"#
                ),
            )],
        }],
    };
