//! Hello-world snippet for `ProgrammingLanguageName::Assembler6809vectrex`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6809vectrex.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%206809vectrex.asm>
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
    language: Assembler6809vectrex => "Assembler 6809vectrex",
    versions: [Assembler6809vectrex => "Assembler 6809vectrex"],
    extensions: [],
}

const ASSEMBLER6809VECTREX: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6809vectrex,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6809vectrex,
            syntax: ASSEMBLER6809VECTREX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER6809VECTREX,
                    [
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" for Vectrex computer with 6809 CPU"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; by Frog ( http://frog.enlight.ru )"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; 24 March 2016"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (
                            TokenKind::Comment,
                            r";***************************************************************************",
                        ),
                        (
                            TokenKind::Plain,
                            r"
                ",
                        ),
                        (TokenKind::Keyword, r"org"),
                        (TokenKind::Plain, r"     "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"


                ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::String, r#""g GCE 2016""#),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"80"),
                        (
                            TokenKind::Plain,
                            r"
                ",
                        ),
                        (TokenKind::Keyword, r"dw"),
                        (
                            TokenKind::Plain,
                            r"      $F600       
                ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      $FC"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"30"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"33"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" -$2a
                ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" $"),
                        (TokenKind::Number, r"80"),
                        (
                            TokenKind::Plain,
                            r" 
                ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"


loop",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"

                ",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r"     $f192

                ",
                        ),
                        (TokenKind::Keyword, r"ldu"),
                        (
                            TokenKind::Plain,
                            r"     #helloworld
                ",
                        ),
                        (TokenKind::Keyword, r"lda"),
                        (TokenKind::Plain, r"     #"),
                        (TokenKind::Number, r"10"),
                        (
                            TokenKind::Plain,
                            r"
                ",
                        ),
                        (TokenKind::Keyword, r"ldb"),
                        (TokenKind::Plain, r"     #-"),
                        (TokenKind::Number, r"45"),
                        (
                            TokenKind::Plain,
                            r"    

                ",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r"     $f37a  

                ",
                        ),
                        (TokenKind::Keyword, r"bra"),
                        (
                            TokenKind::Plain,
                            r"     loop



helloworld",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"     "),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"$"),
                        (TokenKind::Number, r"80"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                    ],
                )
            },
        }],
    };
