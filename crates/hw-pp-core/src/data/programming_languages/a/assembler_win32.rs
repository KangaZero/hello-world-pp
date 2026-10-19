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
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerWin32 => "Assembler (Win32)",
    versions: [AssemblerWin32 => "Assembler (Win32)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerWin32,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerWin32,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
