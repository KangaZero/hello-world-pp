//! Hello-world snippet for `ProgrammingLanguageName::AssemblerWin32`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(Win32).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28Win32%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(Win32)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerWin32 => "Assembler (Win32)",
    versions: [AssemblerWin32 => "Assembler (Win32)"],
    extensions: [],
}

const ASSEMBLER_WIN32: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in Assembler for the Win32 architecture

TITLE ",
    output_for_hello_world!(),
    r" in win32. Tasm

VERSION T310
Model use32 Flat,StdCall

start_code segment byte public 'code' use32
begin:
 Call MessageBox, 0, offset sHallo, offset caption, 0
 Call ExitProcess, 0
start_code Ends

start_data segment byte public 'data' use32

sHallo  db '",
    output_for_hello_world!(),
    r#"',0
caption	db "Hi",0

start_data Ends
End begin"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerWin32,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerWin32,
            syntax: ASSEMBLER_WIN32,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_WIN32,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" in Assembler for the Win32 architecture"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"TITLE"),
                        (
                            TokenKind::Plain,
                            concat!(r" ", output_for_hello_world!(), r" in win32"),
                        ),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r" Tasm

",
                        ),
                        (TokenKind::Keyword, r"VERSION"),
                        (
                            TokenKind::Plain,
                            r" T310
",
                        ),
                        (TokenKind::Keyword, r"Model"),
                        (TokenKind::Plain, r" use32 Flat"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"StdCall

",
                        ),
                        (TokenKind::Keyword, r"start_code"),
                        (TokenKind::Plain, r" segment "),
                        (TokenKind::Keyword, r"byte"),
                        (TokenKind::Plain, r" public "),
                        (TokenKind::String, r"'code'"),
                        (
                            TokenKind::Plain,
                            r" use32
begin",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Keyword, r"Call"),
                        (TokenKind::Plain, r" MessageBox"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" offset sHallo"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" offset caption"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Keyword, r"Call"),
                        (TokenKind::Plain, r" ExitProcess"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"start_code"),
                        (
                            TokenKind::Plain,
                            r" Ends

",
                        ),
                        (TokenKind::Keyword, r"start_data"),
                        (TokenKind::Plain, r" segment "),
                        (TokenKind::Keyword, r"byte"),
                        (TokenKind::Plain, r" public "),
                        (TokenKind::String, r"'data'"),
                        (
                            TokenKind::Plain,
                            r" use32

sHallo  ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
caption	",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""Hi""#),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"start_data"),
                        (
                            TokenKind::Plain,
                            r" Ends
",
                        ),
                        (TokenKind::Keyword, r"End"),
                        (TokenKind::Plain, r" begin"),
                    ],
                )
            },
        }],
    };
