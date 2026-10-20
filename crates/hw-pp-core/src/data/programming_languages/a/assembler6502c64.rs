//! Hello-world snippet for `ProgrammingLanguageName::Assembler6502c64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6502c64.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%206502c64.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler6502c64 => "Assembler 6502c64",
    versions: [Assembler6502c64 => "Assembler 6502c64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502c64,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6502c64,
            syntax: concat!(
                r#"ldy #0
beq in
loop:
jsr $ffd2
iny
in:
lda hello,y
bne loop
rts
hello: .tx ""#,
                output_for_hello_world!(),
                r#""
       .by 13,10,0"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"ldy"),
                (TokenKind::Plain, r" #"),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"beq"),
                (
                    TokenKind::Plain,
                    r" in
loop",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"jsr"),
                (
                    TokenKind::Plain,
                    r" $ffd2
",
                ),
                (TokenKind::Keyword, r"iny"),
                (
                    TokenKind::Plain,
                    r"
in",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"lda"),
                (TokenKind::Plain, r" hello"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"y
",
                ),
                (TokenKind::Keyword, r"bne"),
                (
                    TokenKind::Plain,
                    r" loop
",
                ),
                (TokenKind::Keyword, r"rts"),
                (
                    TokenKind::Plain,
                    r"
hello",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Macro, r".tx"),
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
                (TokenKind::Macro, r".by"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"13"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"10"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"0"),
            ],
        }],
    };
