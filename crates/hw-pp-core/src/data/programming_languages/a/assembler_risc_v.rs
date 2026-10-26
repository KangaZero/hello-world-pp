//! Hello-world snippet for `ProgrammingLanguageName::AssemblerRiscV`.
//!
//! Imported from leachim6/hello-world (`a/Assembler RISC-V.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20RISC-V.s>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerRiscV => "Assembler RISC-V",
    versions: [AssemblerRiscV => "Assembler RISC-V"],
    extensions: [],
}

const ASSEMBLER_RISC_V: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerRiscV,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerRiscV,
            syntax: ASSEMBLER_RISC_V,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_RISC_V,
                    [
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
hello_world",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".asciiz"),
                        (TokenKind::Plain, r" "),
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
                        (TokenKind::Plain, r"   la      a1"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" hello_world
        ",
                        ),
                        (TokenKind::Keyword, r"li"),
                        (TokenKind::Plain, r"      a0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"ecall"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"li"),
                        (TokenKind::Plain, r"      a0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"10"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"ecall"),
                    ],
                )
            },
        }],
    };
