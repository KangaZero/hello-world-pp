//! Hello-world snippet for `ProgrammingLanguageName::Assembler8085`.
//!
//! Imported from leachim6/hello-world (PR #1585 `a/Assembler_8085.asm`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1585>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler8085 => "Assembler 8085",
    versions: [Assembler8085 => "Assembler 8085"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler8085,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler8085,
            syntax: concat!(
                r#"; 8085 Assembly Language program to print ""#,
                output_for_hello_world!(),
                r#""
MESSAGE:			; "#,
                output_for_hello_world!(),
                r#" message string end with 0
    DB ""#,
                output_for_hello_world!(),
                r#"", 0 
START:
     LXI H, MESSAGE  		; Load HL with address of message
    
LOOP:
    MOV A, M  			; Load accumulator with character from memory pointed by HL
    CPI 00H   			; Check if end of string character 0 is reached
    JZ END    			; If end of string, jump to END

    OUT 80H			; Output the data in A to output port 80H that's connected LCD Screen
    INX H     			; Increment HL to point to next character   
    JNZ LOOP  			; loop back to fetch next character

END:
    HLT       			; Halt the processor"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"; 8085 Assembly Language program to print ""#,
                    output_for_hello_world!(),
                    r#""
MESSAGE:			; "#,
                    output_for_hello_world!(),
                    r#" message string end with 0
    DB ""#,
                    output_for_hello_world!(),
                    r#"", 0 
START:
     LXI H, MESSAGE  		; Load HL with address of message
    
LOOP:
    MOV A, M  			; Load accumulator with character from memory pointed by HL
    CPI 00H   			; Check if end of string character 0 is reached
    JZ END    			; If end of string, jump to END

    OUT 80H			; Output the data in A to output port 80H that's connected LCD Screen
    INX H     			; Increment HL to point to next character   
    JNZ LOOP  			; loop back to fetch next character

END:
    HLT       			; Halt the processor"#
                ),
            )],
        }],
    };
