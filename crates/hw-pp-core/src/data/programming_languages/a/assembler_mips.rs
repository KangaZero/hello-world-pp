//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMips`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MIPS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MIPS.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerMips => "Assembler MIPS",
    versions: [AssemblerMips => "Assembler MIPS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMips,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMips,
            syntax: concat!(
                r#".data
    hw:        .asciiz        ""#,
                output_for_hello_world!(),
                r#""
.text
main:
    la $a0, hw  
    li $v0, 4      
    syscall
    li $v0, 10  
    syscall    "#
            ),
            tokenized_syntax: &[
                (TokenKind::Macro, r".data"),
                (
                    TokenKind::Plain,
                    r"
    hw",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"        "),
                (TokenKind::Macro, r".asciiz"),
                (TokenKind::Plain, r"        "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Macro, r".text"),
                (
                    TokenKind::Plain,
                    r"
main",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"la"),
                (TokenKind::Plain, r" $a0"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" hw  
    ",
                ),
                (TokenKind::Keyword, r"li"),
                (TokenKind::Plain, r" $v0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"4"),
                (
                    TokenKind::Plain,
                    r"      
    ",
                ),
                (TokenKind::Keyword, r"syscall"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"li"),
                (TokenKind::Plain, r" $v0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"10"),
                (
                    TokenKind::Plain,
                    r"  
    ",
                ),
                (TokenKind::Keyword, r"syscall"),
                (TokenKind::Plain, r"    "),
            ],
        }],
    };
