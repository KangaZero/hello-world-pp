//! Hello-world snippet for `ProgrammingLanguageName::AssemblerIntel`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Intel.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Intel.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerIntel => "Assembler Intel",
    versions: [AssemblerIntel => "Assembler Intel"],
    extensions: [],
}

const ASSEMBLER_INTEL: &str = concat!(
    r#"
.model small

.stack 100h

.data
msg db ""#,
    output_for_hello_world!(),
    r#"",'$'

.code
main proc
        mov ax,@data
        mov ds, ax


        mov dx,offset msg
        mov ah,09
        int 21h

        mov ax,4c00h
        int 21h

main endp
end main"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerIntel,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerIntel,
            syntax: ASSEMBLER_INTEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_INTEL,
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
                            r" 100h

",
                        ),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
msg ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::String, r"'$'"),
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
                            r" proc
        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ax"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"@data
        mov ",
                        ),
                        (TokenKind::Keyword, r"ds"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" ax


        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" dx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"offset msg
        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ah"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"09"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r" 21h

        ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ax"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"4c00h
        ",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r" 21h

",
                        ),
                        (TokenKind::Keyword, r"main"),
                        (
                            TokenKind::Plain,
                            r" endp
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" main"),
                    ],
                )
            },
        }],
    };
