//! Hello-world snippet for `ProgrammingLanguageName::Assembler6502c64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6502c64.asm` at `a152253`),
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
    language: Assembler6502c64 => "Assembler 6502c64",
    versions: [Assembler6502c64 => "Assembler 6502c64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502c64,
        extensions: &[],
        categories: &[],
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
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
            )],
        }],
    };
