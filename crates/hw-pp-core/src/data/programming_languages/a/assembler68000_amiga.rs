//! Hello-world snippet for `ProgrammingLanguageName::Assembler68000Amiga`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(68000,&nbsp;Amiga)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%2868000%2C%26nbsp%3BAmiga%29>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(68000,%C2%A0Amiga)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler68000Amiga => "Assembler (68000, Amiga)",
    versions: [Assembler68000Amiga => "Assembler (68000, Amiga)"],
    extensions: [],
}

const ASSEMBLER68000_AMIGA: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in 68000 Assembler for dos.library (Amiga)

        move.l  #DOS
        move.l  4.w,a6
        jsr     -$0198(a6)      ;OldOpenLibrary
        move.l  d0,a6
        beq.s   .Out
        move.l  #HelloWorld,d1

A)      moveq   #13,d2
        jsr     -$03AE(a6)      ;WriteChars

B)      jsr     -$03B4          ;PutStr

        move.l  a6,a1
        move.l  4.w,a6
        jsr     -$019E(a6)      ;CloseLibrary
.Out    rts

DOS          dc.b    'dos.library',0
HelloWorld   dc.b    '",
    output_for_hello_world!(),
    r"',$A,0"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler68000Amiga,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler68000Amiga,
            syntax: ASSEMBLER68000_AMIGA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER68000_AMIGA,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" in 68000 Assembler for dos.library (Amiga)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (
                            TokenKind::Plain,
                            r"  #DOS
        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"  "),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Macro, r".w"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"a6
        ",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"     -$"),
                        (TokenKind::Number, r"0198"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"a6"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Comment, r";OldOpenLibrary"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"  d0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"a6
        ",
                        ),
                        (TokenKind::Keyword, r"beq"),
                        (TokenKind::Macro, r".s"),
                        (TokenKind::Plain, r"   "),
                        (TokenKind::Macro, r".Out"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"  #HelloWorld"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"d1

",
                        ),
                        (TokenKind::Keyword, r"A"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"      moveq   #"),
                        (TokenKind::Number, r"13"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"d2
        ",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"     -$03AE"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"a6"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Comment, r";WriteChars"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"B"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"      jsr     -$03B4          "),
                        (TokenKind::Comment, r";PutStr"),
                        (
                            TokenKind::Plain,
                            r"

        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"  a6"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"a1
        ",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"  "),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Macro, r".w"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"a6
        ",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"     -$019E"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"a6"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Comment, r";CloseLibrary"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".Out"),
                        (
                            TokenKind::Plain,
                            r"    rts

DOS          ",
                        ),
                        (TokenKind::Keyword, r"dc"),
                        (TokenKind::Macro, r".b"),
                        (TokenKind::Plain, r"    "),
                        (TokenKind::String, r"'dos.library'"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
HelloWorld   ",
                        ),
                        (TokenKind::Keyword, r"dc"),
                        (TokenKind::Macro, r".b"),
                        (TokenKind::Plain, r"    "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"$A"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                    ],
                )
            },
        }],
    };
