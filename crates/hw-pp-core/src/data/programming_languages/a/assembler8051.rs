//! Hello-world snippet for `ProgrammingLanguageName::Assembler8051`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 8051.asm` at `a152253`),
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
    language: Assembler8051 => "Assembler 8051",
    versions: [Assembler8051 => "Assembler 8051"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler8051,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler8051,
            syntax: concat!(
                r"ORG 00H
SJMP INIT

INIT:
; Timer1 as the UART1 buadrate generator
        MOV TMOD, #20H  ; Timer1, 8-bit auto-reload mode
        MOV TH1,  #0FDH ; 9600 baudrate at 11.0592MHz
        MOV SCON, #50H  ; Serial mode 1, enable reception
        SETB TR1        ; Start Timer1
        MOV DPTR, #20H  ; Point DPTR to the start of the string

SEND:
        CLR A
        MOVC A, @A+DPTR ; Get the next character
        INC DPTR
        JZ DONE         ; End program if the character is null
        MOV SBUF, A     ; Send the character
        JNB TI, $       ; Wait for transmission to complete
        CLR TI          ; Clear TI flag
        SJMP SEND

DONE:
        SJMP $          ; Endless loop

ORG 20H
DB '",
                output_for_hello_world!(),
                r"', 0

END"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"ORG 00H
SJMP INIT

INIT:
; Timer1 as the UART1 buadrate generator
        MOV TMOD, #20H  ; Timer1, 8-bit auto-reload mode
        MOV TH1,  #0FDH ; 9600 baudrate at 11.0592MHz
        MOV SCON, #50H  ; Serial mode 1, enable reception
        SETB TR1        ; Start Timer1
        MOV DPTR, #20H  ; Point DPTR to the start of the string

SEND:
        CLR A
        MOVC A, @A+DPTR ; Get the next character
        INC DPTR
        JZ DONE         ; End program if the character is null
        MOV SBUF, A     ; Send the character
        JNB TI, $       ; Wait for transmission to complete
        CLR TI          ; Clear TI flag
        SJMP SEND

DONE:
        SJMP $          ; Endless loop

ORG 20H
DB '",
                    output_for_hello_world!(),
                    r"', 0

END"
                ),
            )],
        }],
    };
