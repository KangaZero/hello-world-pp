//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArmAarch64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler ARM aarch64.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20ARM%20aarch64.s>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerArmAarch64 => "Assembler ARM aarch64",
    versions: [AssemblerArmAarch64 => "Assembler ARM aarch64"],
    extensions: [],
}

const ASSEMBLER_ARM_AARCH64: &str = concat!(
    r#".data

msg:
    .ascii        ""#,
    output_for_hello_world!(),
    r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     x0, #1
    ldr     x1, =msg
    ldr     x2, =len
    mov     w8, #64
    svc     #0

    mov     x0, #0
    mov     w8, #93
    svc     #0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArmAarch64,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArmAarch64,
            syntax: ASSEMBLER_ARM_AARCH64,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ARM_AARCH64,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#".data

msg:
    .ascii        ""#,
                            output_for_hello_world!(),
                            r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     x0, #1
    ldr     x1, =msg
    ldr     x2, =len
    mov     w8, #64
    svc     #0

    mov     x0, #0
    mov     w8, #93
    svc     #0"#
                        ),
                    )],
                )
            },
        }],
    };
