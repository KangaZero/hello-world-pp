//! Hello-world snippet for `ProgrammingLanguageName::Assembler6809vectrex`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6809vectrex.asm` at `a152253`),
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
    language: Assembler6809vectrex => "Assembler 6809vectrex",
    versions: [Assembler6809vectrex => "Assembler 6809vectrex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6809vectrex,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6809vectrex,
            syntax: concat!(
                r"
; ",
                output_for_hello_world!(),
                r#" for Vectrex computer with 6809 CPU
; by Frog ( http://frog.enlight.ru )
; 24 March 2016

;***************************************************************************
                org     0


                db      "g GCE 2016", $80
                dw      $F600       
                db      $FC, $30, 33, -$2a
                db      ""#,
                output_for_hello_world!(),
                r#"", $80 
                db      0


loop:

                jsr     $f192

                ldu     #helloworld
                lda     #10
                ldb     #-45    

                jsr     $f37a  

                bra     loop



helloworld:     db      '"#,
                output_for_hello_world!(),
                r"',$80
"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"
; ",
                    output_for_hello_world!(),
                    r#" for Vectrex computer with 6809 CPU
; by Frog ( http://frog.enlight.ru )
; 24 March 2016

;***************************************************************************
                org     0


                db      "g GCE 2016", $80
                dw      $F600       
                db      $FC, $30, 33, -$2a
                db      ""#,
                    output_for_hello_world!(),
                    r#"", $80 
                db      0


loop:

                jsr     $f192

                ldu     #helloworld
                lda     #10
                ldb     #-45    

                jsr     $f37a  

                bra     loop



helloworld:     db      '"#,
                    output_for_hello_world!(),
                    r"',$80
"
                ),
            )],
        }],
    };
