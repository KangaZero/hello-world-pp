//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ390`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(z390).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28z390%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(z390)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ390 => "Assembler (z390)",
    versions: [AssemblerZ390 => "Assembler (z390)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ390,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ390,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r" for z390 IBM compatible mainframe assembler

HELLO CSECT
     USING *,15
     WTO '",
                output_for_hello_world!(),
                r"'
     BR 14
     END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r" for z390 IBM compatible mainframe assembler

HELLO CSECT
     USING *,15
     WTO '",
                    output_for_hello_world!(),
                    r"'
     BR 14
     END"
                ),
            )],
        }],
    };
