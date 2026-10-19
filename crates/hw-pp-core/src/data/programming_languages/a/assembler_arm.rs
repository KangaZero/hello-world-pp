//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArm`.
//!
//! Imported from leachim6/hello-world (`a/Assembler ARM.s` at `a152253`),
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
    language: AssemblerArm => "Assembler ARM",
    versions: [AssemblerArm => "Assembler ARM"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArm,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArm,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };
