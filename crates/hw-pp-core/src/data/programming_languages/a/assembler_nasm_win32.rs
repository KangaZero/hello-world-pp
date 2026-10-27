//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmWin32`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Win32.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20Win32.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerNasmWin32 => "Assembler NASM Win32",
    versions: [AssemblerNasmWin32 => "Assembler NASM Win32"],
    extensions: [],
}

const ASSEMBLER_NASM_WIN32: &str = concat!(
    r#"BITS	32
global	_main
extern	_printf
section	.text
	push	msg
	call	_printf
	add		esp,4
	ret
section	.data
	msg:	db	""#,
    output_for_hello_world!(),
    r#"",0Ah,0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmWin32,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmWin32,
            syntax: ASSEMBLER_NASM_WIN32,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_NASM_WIN32,
                    [
                        (TokenKind::Keyword, r"BITS"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"32"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"global"),
                        (
                            TokenKind::Plain,
                            r"	_main
",
                        ),
                        (TokenKind::Keyword, r"extern"),
                        (
                            TokenKind::Plain,
                            r"	_printf
",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".text"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r"	msg
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	_printf
	",
                        ),
                        (TokenKind::Keyword, r"add"),
                        (TokenKind::Plain, r"		esp"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"ret"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"
	msg",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"0Ah"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                    ],
                )
            },
        }],
    };
