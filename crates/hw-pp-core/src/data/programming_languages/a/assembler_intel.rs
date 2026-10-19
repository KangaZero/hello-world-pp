//! Hello-world snippet for `ProgrammingLanguageName::AssemblerIntel`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Intel.asm` at `a152253`),
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
    language: AssemblerIntel => "Assembler Intel",
    versions: [AssemblerIntel => "Assembler Intel"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerIntel,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerIntel,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
