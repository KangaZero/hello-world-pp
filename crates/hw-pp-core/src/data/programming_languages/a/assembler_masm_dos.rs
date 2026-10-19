//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMasmDos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MASM DOS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerMasmDos => "Assembler MASM DOS",
    versions: [AssemblerMasmDos => "Assembler MASM DOS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMasmDos,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMasmDos,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
