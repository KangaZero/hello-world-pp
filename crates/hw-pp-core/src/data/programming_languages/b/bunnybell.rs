//! Hello-world snippet for `ProgrammingLanguageName::Bunnybell`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BunnyBell>
//! - <https://esolangs.org/wiki/BunnyBell>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bunnybell => "BunnyBell",
    versions: [Bunnybell => "BunnyBell"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bunnybell,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bunnybell,
            syntax: concat!(
                r"func main
 print ",
                output_for_hello_world!(),
                r"
 return"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"func main
 print ",
                    output_for_hello_world!(),
                    r"
 return"
                ),
            )],
        }],
    };
