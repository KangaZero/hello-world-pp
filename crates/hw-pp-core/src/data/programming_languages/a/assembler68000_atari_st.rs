//! Hello-world snippet for `ProgrammingLanguageName::Assembler68000AtariSt`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(68000,&nbsp;Atari&nbsp;ST)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%2868000%2C%26nbsp%3BAtari%26nbsp%3BST%29>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(68000,%C2%A0Atari%C2%A0ST)>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Assembler68000AtariSt => "Assembler (68000, Atari ST)",
    versions: [Assembler68000AtariSt => "Assembler (68000, Atari ST)"],
    extensions: [],
}

const ASSEMBLER68000_ATARI_ST: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler68000AtariSt,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler68000AtariSt,
            syntax: ASSEMBLER68000_ATARI_ST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER68000_ATARI_ST,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" in 68000 Assembler (Atari ST)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

     ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r" #helloworld"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"-"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"A7"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Plain, r"   #"),
                        (TokenKind::Number, r"9"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"-"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"A7"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"trap"),
                        (TokenKind::Plain, r"   #"),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"addq"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"6"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"A7
     ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Plain, r"   #"),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"-"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"A7"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"trap"),
                        (TokenKind::Plain, r"   #"),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
helloworld",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"dc"),
                        (TokenKind::Macro, r".b"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"$0d"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"$0a"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                    ],
                )
            },
        }],
    };
