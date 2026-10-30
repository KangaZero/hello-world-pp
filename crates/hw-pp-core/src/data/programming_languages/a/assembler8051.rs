//! Hello-world snippet for `ProgrammingLanguageName::Assembler8051`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 8051.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%208051.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Assembler8051 => "Assembler 8051",
    versions: [Assembler8051 => "Assembler 8051"],
    extensions: [],
}

const ASSEMBLER8051: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler8051,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler8051,
            syntax: ASSEMBLER8051,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER8051,
                    [
                        (TokenKind::Keyword, r"ORG"),
                        (
                            TokenKind::Plain,
                            r" 00H
",
                        ),
                        (TokenKind::Keyword, r"SJMP"),
                        (
                            TokenKind::Plain,
                            r" INIT

INIT",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; Timer1 as the UART1 buadrate generator",
                        ),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" TMOD"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" #20H  "),
                        (TokenKind::Comment, r"; Timer1, 8-bit auto-reload mode"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" TH1"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"  #0FDH "),
                        (TokenKind::Comment, r"; 9600 baudrate at 11.0592MHz"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" SCON"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" #50H  "),
                        (TokenKind::Comment, r"; Serial mode 1, enable reception"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"SETB"),
                        (TokenKind::Plain, r" TR1        "),
                        (TokenKind::Comment, r"; Start Timer1"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" DPTR"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" #20H  "),
                        (
                            TokenKind::Comment,
                            r"; Point DPTR to the start of the string",
                        ),
                        (
                            TokenKind::Plain,
                            r"

SEND",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"CLR"),
                        (
                            TokenKind::Plain,
                            r" A
        ",
                        ),
                        (TokenKind::Keyword, r"MOVC"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" @A+DPTR "),
                        (TokenKind::Comment, r"; Get the next character"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" DPTR
        ",
                        ),
                        (TokenKind::Keyword, r"JZ"),
                        (TokenKind::Plain, r" DONE         "),
                        (
                            TokenKind::Comment,
                            r"; End program if the character is null",
                        ),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" SBUF"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" A     "),
                        (TokenKind::Comment, r"; Send the character"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"JNB"),
                        (TokenKind::Plain, r" TI"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" $       "),
                        (TokenKind::Comment, r"; Wait for transmission to complete"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"CLR"),
                        (TokenKind::Plain, r" TI          "),
                        (TokenKind::Comment, r"; Clear TI flag"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"SJMP"),
                        (
                            TokenKind::Plain,
                            r" SEND

DONE",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"SJMP"),
                        (TokenKind::Plain, r" $          "),
                        (TokenKind::Comment, r"; Endless loop"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"ORG"),
                        (
                            TokenKind::Plain,
                            r" 20H
",
                        ),
                        (TokenKind::Keyword, r"DB"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"END"),
                    ],
                )
            },
        }],
    };
