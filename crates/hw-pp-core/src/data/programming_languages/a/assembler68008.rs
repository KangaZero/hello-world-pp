//! Hello-world snippet for `ProgrammingLanguageName::Assembler68008`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(68008)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%2868008%29>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(68008)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler68008 => "Assembler (68008)",
    versions: [Assembler68008 => "Assembler (68008)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler68008,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler68008,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r" in 68008 Assembler (Sinclar QL)

     move.l #0,a0
     lea.l  mess,a1
     move.w $d0,a2
     jsr    (a2)
     rts
mess dc.w   12
     dc.b   '",
                output_for_hello_world!(),
                r"',10
     end"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r" in 68008 Assembler (Sinclar QL)

     move.l #0,a0
     lea.l  mess,a1
     move.w $d0,a2
     jsr    (a2)
     rts
mess dc.w   12
     dc.b   '",
                    output_for_hello_world!(),
                    r"',10
     end"
                ),
            )],
        }],
    };
