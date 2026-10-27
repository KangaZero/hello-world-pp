//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArm`.
//!
//! Imported from leachim6/hello-world (`a/Assembler ARM.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20ARM.s>
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
    language: AssemblerArm => "Assembler ARM",
    versions: [AssemblerArm => "Assembler ARM"],
    extensions: [],
}

const ASSEMBLER_ARM: &str = concat!(
    r#".data
message:                .asciz ""#,
    output_for_hello_world!(),
    r#""
len = .-message

.text
.global main
main:
    mov r0, #1  
    ldr r1, =message
    ldr r2, =len
    mov r7, #4    
    swi 0           

    mov r7, #1
    swi 0
    bx lr
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArm,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArm,
            syntax: ASSEMBLER_ARM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ARM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#".data
message:                .asciz ""#,
                            output_for_hello_world!(),
                            r#""
len = .-message

.text
.global main
main:
    mov r0, #1  
    ldr r1, =message
    ldr r2, =len
    mov r7, #4    
    swi 0           

    mov r7, #1
    swi 0
    bx lr
"#
                        ),
                    )],
                )
            },
        }],
    };
