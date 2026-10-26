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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler8085 => "Assembler 8085",
    versions: [Assembler8085 => "Assembler 8085"],
    extensions: [],
}

const ASSEMBLER8085: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler8085,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler8085,
            syntax: ASSEMBLER8085,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER8085,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"; 8085 Assembly Language program to print ""#,
                                output_for_hello_world!(),
                                r#"""#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
MESSAGE",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"			"),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" message string end with 0"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"DB"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r" 
START",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"LXI"),
                        (TokenKind::Plain, r" H"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" MESSAGE  		"),
                        (TokenKind::Comment, r"; Load HL with address of message"),
                        (
                            TokenKind::Plain,
                            r"
    
LOOP",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"MOV"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" M  			"),
                        (
                            TokenKind::Comment,
                            r"; Load accumulator with character from memory pointed by HL",
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"CPI"),
                        (TokenKind::Plain, r" 00H   			"),
                        (
                            TokenKind::Comment,
                            r"; Check if end of string character 0 is reached",
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"JZ"),
                        (TokenKind::Plain, r" END    			"),
                        (TokenKind::Comment, r"; If end of string, jump to END"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r" 80H			"),
                        (
                            TokenKind::Comment,
                            r"; Output the data in A to output port 80H that's connected LCD Screen",
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"INX"),
                        (TokenKind::Plain, r" H     			"),
                        (
                            TokenKind::Comment,
                            r"; Increment HL to point to next character   ",
                        ),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"JNZ"),
                        (TokenKind::Plain, r" LOOP  			"),
                        (TokenKind::Comment, r"; loop back to fetch next character"),
                        (
                            TokenKind::Plain,
                            r"

END",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"HLT"),
                        (TokenKind::Plain, r"       			"),
                        (TokenKind::Comment, r"; Halt the processor"),
                    ],
                )
            },
        }],
    };
