//! Hello-world snippet for `ProgrammingLanguageName::Bauberqueue`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bauberqueue>
//! - <https://esolangs.org/wiki/Bauberqueue>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bauberqueue => "Bauberqueue",
    versions: [Bauberqueue => "Bauberqueue"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bauberqueue,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bauberqueue,
            syntax: concat!(
                r"=b1=bB\x11=a1=aA\x19+aB-b1iia=a1+a1=oA=a1=aA\x19+aB-b1iia
",
                output_for_hello_world!(),
                r"\n
!"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"=b1=bB\x11=a1=aA\x19+aB-b1iia=a1+a1=oA=a1=aA\x19+aB-b1iia
",
                    output_for_hello_world!(),
                    r"\n
!"
                ),
            )],
        }],
    };
