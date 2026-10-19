//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Knightos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 Knightos.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Z80%20Knightos.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ80Knightos => "Assembler Z80 Knightos",
    versions: [AssemblerZ80Knightos => "Assembler Z80 Knightos"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Knightos,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Knightos,
            syntax: concat!(
                r#"#include "kernel.inc"
    .db "KEXC"
    .db KEXC_ENTRY_POINT
    .dw start
start:
    pcall(getLcdLock)

    pcall(allocScreenBuffer)
    pcall(clearScreen)

    kld(hl, message)
    ld de, 0
    pcall(drawStr)

message:
    .db ""#,
                output_for_hello_world!(),
                r#"", 0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include "kernel.inc"
    .db "KEXC"
    .db KEXC_ENTRY_POINT
    .dw start
start:
    pcall(getLcdLock)

    pcall(allocScreenBuffer)
    pcall(clearScreen)

    kld(hl, message)
    ld de, 0
    pcall(drawStr)

message:
    .db ""#,
                    output_for_hello_world!(),
                    r#"", 0"#
                ),
            )],
        }],
    };
