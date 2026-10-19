//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZx81`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(ZX81).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28ZX81%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(ZX81)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZx81 => "Assembler (ZX81)",
    versions: [AssemblerZx81 => "Assembler (ZX81)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZx81,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZx81,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r" in Assembler for the ZX81 (Zilog Z80)

          CALL SPRINT
          DEFM ",
                output_for_hello_world!(),
                r".
          DEFB FF
          RET
SPRINT    POP HL
          LD A,(HL)
          INC HL
          PUSH HL
          CP FF
          RET Z
          CALL PRINT
          JR SPRINT"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r" in Assembler for the ZX81 (Zilog Z80)

          CALL SPRINT
          DEFM ",
                    output_for_hello_world!(),
                    r".
          DEFB FF
          RET
SPRINT    POP HL
          LD A,(HL)
          INC HL
          PUSH HL
          CP FF
          RET Z
          CALL PRINT
          JR SPRINT"
                ),
            )],
        }],
    };
