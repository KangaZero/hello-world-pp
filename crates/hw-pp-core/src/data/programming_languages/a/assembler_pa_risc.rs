//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPaRisc`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(PA-RISC).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28PA-RISC%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(PA-RISC)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerPaRisc => "Assembler (PA-RISC)",
    versions: [AssemblerPaRisc => "Assembler (PA-RISC)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPaRisc,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPaRisc,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" written in PA-RISC 2.0 assembly code

    .LEVEL  2.0N

    .SPACE  $TEXT$,SORT=8
    .SUBSPA $CODE$,QUAD=0,ALIGN=4,ACCESS=0x2c,CODE_ONLY,SORT=24
main
    .PROC
    .CALLINFO CALLER,FRAME=16,SAVE_RP,ORDERING_AWARE
        .ENTRY
        STW     %r2,-20(%r30)   ;offset 0x0
        LDO     64(%r30),%r30   ;offset 0x4
        ADDIL   LR'M$3-$global$,%r27,%r1        ;offset 0x8
        LDO     RR'M$3-$global$(%r1),%r1        ;offset 0xc
        STW     %r1,-56(%r30)   ;offset 0x10
        ADDIL   LR'M$3-$global$+16,%r27,%r1     ;offset 0x14
        LDO     RR'M$3-$global$+16(%r1),%r26    ;offset 0x18
        LDW     -56(%r30),%r25  ;offset 0x1c
        LDIL    L'printf,%r31   ;offset 0x20
        .CALL   ARGW0=GR,ARGW1=GR,RTNVAL=GR     ;in=25,26;out=28;
        BE,L    R'printf(%sr4,%r31),%r31        ;offset 0x24
        COPY    %r31,%r2        ;offset 0x28
        LDW     -84(%r30),%r2   ;offset 0x2c
        BVE     (%r2)   ;offset 0x30
        .EXIT
        LDO     -64(%r30),%r30  ;offset 0x34
    .PROCEND    ;


    .SPACE  $TEXT$
    .SUBSPA $CODE$
    .SPACE  $PRIVATE$,SORT=16
    .SUBSPA $DATA$,QUAD=1,ALIGN=8,ACCESS=0x1f,SORT=16
M$3
    .ALIGN  8
    .STRINGZ    ""#,
                output_for_hello_world!(),
                r#""
    .BLOCK  4
    .STRINGZ    "%s\n"
    .IMPORT $global$,DATA
    .SPACE  $TEXT$
    .SUBSPA $CODE$
    .EXPORT main,ENTRY,PRIV_LEV=3,LONG_RETURN
    .IMPORT printf,CODE
    .END"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    concat!(r"// ", output_for_hello_world!(), r" written in PA-RISC "),
                ),
                (TokenKind::Number, r"2.0"),
                (
                    TokenKind::Plain,
                    r" assembly code

    ",
                ),
                (TokenKind::Macro, r".LEVEL"),
                (TokenKind::Plain, r"  "),
                (TokenKind::Number, r"2"),
                (TokenKind::Macro, r".0N"),
                (
                    TokenKind::Plain,
                    r"

    ",
                ),
                (TokenKind::Macro, r".SPACE"),
                (TokenKind::Plain, r"  $"),
                (TokenKind::Keyword, r"TEXT"),
                (TokenKind::Plain, r"$"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"SORT="),
                (TokenKind::Number, r"8"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".SUBSPA"),
                (TokenKind::Plain, r" $CODE$"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"QUAD="),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ALIGN="),
                (TokenKind::Number, r"4"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ACCESS="),
                (TokenKind::Number, r"0x2c"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"CODE_ONLY"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"SORT="),
                (TokenKind::Number, r"24"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"main"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".PROC"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".CALLINFO"),
                (TokenKind::Plain, r" CALLER"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"FRAME="),
                (TokenKind::Number, r"16"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"SAVE_RP"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"ORDERING_AWARE
        ",
                ),
                (TokenKind::Macro, r".ENTRY"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"STW"),
                (TokenKind::Plain, r"     %r2"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"-"),
                (TokenKind::Number, r"20"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r"   "),
                (TokenKind::Comment, r";offset 0x0"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDO"),
                (TokenKind::Plain, r"     "),
                (TokenKind::Number, r"64"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r"%r30   "),
                (TokenKind::Comment, r";offset 0x4"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"ADDIL"),
                (TokenKind::Plain, r"   LR"),
                (
                    TokenKind::String,
                    r"'M$3-$global$,%r27,%r1        ;offset 0x8",
                ),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDO"),
                (TokenKind::Plain, r"     RR"),
                (
                    TokenKind::String,
                    r"'M$3-$global$(%r1),%r1        ;offset 0xc",
                ),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"STW"),
                (TokenKind::Plain, r"     %r1"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"-"),
                (TokenKind::Number, r"56"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r"   "),
                (TokenKind::Comment, r";offset 0x10"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"ADDIL"),
                (TokenKind::Plain, r"   LR"),
                (
                    TokenKind::String,
                    r"'M$3-$global$+16,%r27,%r1     ;offset 0x14",
                ),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDO"),
                (TokenKind::Plain, r"     RR"),
                (
                    TokenKind::String,
                    r"'M$3-$global$+16(%r1),%r26    ;offset 0x18",
                ),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDW"),
                (TokenKind::Plain, r"     -"),
                (TokenKind::Number, r"56"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r"%r25  "),
                (TokenKind::Comment, r";offset 0x1c"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDIL"),
                (TokenKind::Plain, r"    L"),
                (TokenKind::String, r"'printf,%r31   ;offset 0x20"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Macro, r".CALL"),
                (TokenKind::Plain, r"   ARGW0=GR"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ARGW1=GR"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"RTNVAL=GR     "),
                (TokenKind::Comment, r";in=25,26;out=28;"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"BE"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"L    R"),
                (
                    TokenKind::String,
                    r"'printf(%sr4,%r31),%r31        ;offset 0x24",
                ),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"COPY"),
                (TokenKind::Plain, r"    %r31"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"%r2        "),
                (TokenKind::Comment, r";offset 0x28"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDW"),
                (TokenKind::Plain, r"     -"),
                (TokenKind::Number, r"84"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r"%r2   "),
                (TokenKind::Comment, r";offset 0x2c"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"BVE"),
                (TokenKind::Plain, r"     "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r2"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r"   "),
                (TokenKind::Comment, r";offset 0x30"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Macro, r".EXIT"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"LDO"),
                (TokenKind::Plain, r"     -"),
                (TokenKind::Number, r"64"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"%r30"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r"%r30  "),
                (TokenKind::Comment, r";offset 0x34"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".PROCEND"),
                (TokenKind::Plain, r"    "),
                (TokenKind::Comment, r";"),
                (
                    TokenKind::Plain,
                    r"


    ",
                ),
                (TokenKind::Macro, r".SPACE"),
                (TokenKind::Plain, r"  $"),
                (TokenKind::Keyword, r"TEXT"),
                (
                    TokenKind::Plain,
                    r"$
    ",
                ),
                (TokenKind::Macro, r".SUBSPA"),
                (
                    TokenKind::Plain,
                    r" $CODE$
    ",
                ),
                (TokenKind::Macro, r".SPACE"),
                (TokenKind::Plain, r"  $PRIVATE$"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"SORT="),
                (TokenKind::Number, r"16"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".SUBSPA"),
                (TokenKind::Plain, r" $DATA$"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"QUAD="),
                (TokenKind::Number, r"1"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ALIGN="),
                (TokenKind::Number, r"8"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ACCESS="),
                (TokenKind::Number, r"0x1f"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"SORT="),
                (TokenKind::Number, r"16"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"M"),
                (TokenKind::Plain, r"$"),
                (TokenKind::Number, r"3"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".ALIGN"),
                (TokenKind::Plain, r"  "),
                (TokenKind::Number, r"8"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".STRINGZ"),
                (TokenKind::Plain, r"    "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".BLOCK"),
                (TokenKind::Plain, r"  "),
                (TokenKind::Number, r"4"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".STRINGZ"),
                (TokenKind::Plain, r"    "),
                (TokenKind::String, r#""%s\n""#),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Macro, r".IMPORT"),
                (TokenKind::Plain, r" $global$"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"DATA
    ",
                ),
                (TokenKind::Macro, r".SPACE"),
                (TokenKind::Plain, r"  $"),
                (TokenKind::Keyword, r"TEXT"),
                (
                    TokenKind::Plain,
                    r"$
    ",
                ),
                (TokenKind::Macro, r".SUBSPA"),
                (
                    TokenKind::Plain,
                    r" $CODE$
    ",
                ),
                (TokenKind::Macro, r".EXPORT"),
                (TokenKind::Plain, r" main"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"ENTRY"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"PRIV_LEV="),
                (TokenKind::Number, r"3"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"LONG_RETURN
    ",
                ),
                (TokenKind::Macro, r".IMPORT"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"printf"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"CODE
    ",
                ),
                (TokenKind::Macro, r".END"),
            ],
        }],
    };
