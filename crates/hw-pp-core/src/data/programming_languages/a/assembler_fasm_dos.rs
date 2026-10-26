//! Hello-world snippet for `ProgrammingLanguageName::AssemblerFasmDos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler FASM DOS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20FASM%20DOS.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerFasmDos => "Assembler FASM DOS",
    versions: [AssemblerFasmDos => "Assembler FASM DOS"],
    extensions: [],
}

const ASSEMBLER_FASM_DOS: &str = concat!(
    r"format MZ

entry .code: start

segment .code
start:
	mov ax, .data
	mov ds, ax
	mov dx, msg 
	mov ah, 9h
	int 21h
	mov ah, 4ch
	int 21h

segment .data
	msg db '",
    output_for_hello_world!(),
    r"', '$'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerFasmDos,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerFasmDos,
            syntax: ASSEMBLER_FASM_DOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_FASM_DOS,
                    [
                        (TokenKind::Keyword, r"format"),
                        (
                            TokenKind::Plain,
                            r" MZ

",
                        ),
                        (TokenKind::Keyword, r"entry"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".code"),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r" start

",
                        ),
                        (TokenKind::Keyword, r"segment"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".code"),
                        (
                            TokenKind::Plain,
                            r"
start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
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
                            r" msg 
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ah"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" 9h
	",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r" 21h
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" ah"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" 4ch
	",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (
                            TokenKind::Plain,
                            r" 21h

",
                        ),
                        (TokenKind::Keyword, r"segment"),
                        (TokenKind::Plain, r" "),
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
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'$'"),
                    ],
                )
            },
        }],
    };
