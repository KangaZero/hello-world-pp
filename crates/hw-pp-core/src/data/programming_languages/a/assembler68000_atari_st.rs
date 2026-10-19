//! Hello-world snippet for `ProgrammingLanguageName::Assembler68000AtariSt`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(68000,&nbsp;Atari&nbsp;ST)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%2868000%2C%26nbsp%3BAtari%26nbsp%3BST%29>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(68000,%C2%A0Atari%C2%A0ST)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler68000AtariSt => "Assembler (68000, Atari ST)",
    versions: [Assembler68000AtariSt => "Assembler (68000, Atari ST)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler68000AtariSt,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler68000AtariSt,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in 68000 Assembler (Atari ST)

     move.l #helloworld,-(A7)
     move   #9,-(A7)
     trap   #1
     addq.l #6,A7
     move   #0,-(A7)
     trap   #1
helloworld:
     dc.b ""#,
                output_for_hello_world!(),
                r#"",$0d,$0a,0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in 68000 Assembler (Atari ST)

     move.l #helloworld,-(A7)
     move   #9,-(A7)
     trap   #1
     addq.l #6,A7
     move   #0,-(A7)
     trap   #1
helloworld:
     dc.b ""#,
                    output_for_hello_world!(),
                    r#"",$0d,$0a,0"#
                ),
            )],
        }],
    };
