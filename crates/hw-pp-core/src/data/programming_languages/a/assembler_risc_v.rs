//! Hello-world snippet for `ProgrammingLanguageName::AssemblerRiscV`.
//!
//! Imported from leachim6/hello-world (`a/Assembler RISC-V.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerRiscV => "Assembler RISC-V",
    versions: [AssemblerRiscV => "Assembler RISC-V"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerRiscV,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerRiscV,
            syntax: concat!(
                r#".data
hello_world: .asciiz ""#,
                output_for_hello_world!(),
                r#""

.text
main:   la      a1, hello_world
        li      a0, 4
        ecall

        li      a0, 10
        ecall"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#".data
hello_world: .asciiz ""#,
                    output_for_hello_world!(),
                    r#""

.text
main:   la      a1, hello_world
        li      a0, 4
        ecall

        li      a0, 10
        ecall"#
                ),
            )],
        }],
    };
