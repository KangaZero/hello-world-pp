//! Hello-world snippet for `ProgrammingLanguageName::CopypastaLanguage`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CopyPasta_Language>
//! - <https://esolangs.org/wiki/CopyPasta_Language>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CopypastaLanguage => "CopyPasta Language",
    versions: [CopypastaLanguage => "CopyPasta Language"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CopypastaLanguage,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CopypastaLanguage,
            syntax: concat!(
                r"Copy
",
                output_for_hello_world!(),
                r"
Pasta!"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"Copy
",
                    output_for_hello_world!(),
                    r"
Pasta!"
                ),
            )],
        }],
    };
