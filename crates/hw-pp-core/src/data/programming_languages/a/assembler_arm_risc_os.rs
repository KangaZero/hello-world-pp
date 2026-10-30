//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArmRiscOs`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(ARM,&nbsp;RISC&nbsp;OS).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28ARM%2C%26nbsp%3BRISC%26nbsp%3BOS%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(ARM,%C2%A0RISC%C2%A0OS)>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AssemblerArmRiscOs => "Assembler (ARM, RISC OS)",
    versions: [AssemblerArmRiscOs => "Assembler (ARM, RISC OS)"],
    extensions: [],
}

const ASSEMBLER_ARM_RISC_OS: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in ARM code assembler, with RISC OS software interrupt

SWI "OS_WriteS"
EQUS ""#,
    output_for_hello_world!(),
    r#""
EQUB 0
ALIGN
MOV PC,R14"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArmRiscOs,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArmRiscOs,
            syntax: ASSEMBLER_ARM_RISC_OS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ARM_RISC_OS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in ARM code assembler, with RISC OS software interrupt

SWI "OS_WriteS"
EQUS ""#,
                            output_for_hello_world!(),
                            r#""
EQUB 0
ALIGN
MOV PC,R14"#
                        ),
                    )],
                )
            },
        }],
    };
