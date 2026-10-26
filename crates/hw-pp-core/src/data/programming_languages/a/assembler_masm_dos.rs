//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMasmDos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MASM DOS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MASM%20DOS.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerMasmDos => "Assembler MASM DOS",
    versions: [AssemblerMasmDos => "Assembler MASM DOS"],
    extensions: [],
}

const ASSEMBLER_MASM_DOS: &str = concat!(
    r#"
.model small
.stack
.data
    message   db ""#,
    output_for_hello_world!(),
    r#"", "$"
.code
    main    proc
        mov   ax,seg message
        mov   ds,ax
        mov   ah,09
        lea   dx,message
        int   21h
        
        mov   ax,4c00h
        int   21h
    main    endp
end main"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMasmDos,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMasmDos,
            syntax: ASSEMBLER_MASM_DOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_MASM_DOS,
                    [
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".model"),
                        (
                            TokenKind::Plain,
                            r" small
",
                        ),
                        (TokenKind::Macro, r".stack"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
    message   ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""$""#),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".code"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"main"),
                        (
                            TokenKind::Plain,
                            r"    proc
        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"   ax"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"seg message
        mov   ",
                        ),
                        (TokenKind::Keyword, r"ds"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"ax
        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"   ah"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"09"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"lea"),
                        (TokenKind::Plain, r"   dx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"message
        ",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r"   21h
        
        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"   ax"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"4c00h
        ",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r"   21h
    ",
                        ),
                        (TokenKind::Keyword, r"main"),
                        (
                            TokenKind::Plain,
                            r"    endp
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" main"),
                    ],
                )
            },
        }],
    };
