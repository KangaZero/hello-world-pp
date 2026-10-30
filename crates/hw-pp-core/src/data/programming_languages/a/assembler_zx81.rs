//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZx81`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(ZX81).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28ZX81%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(ZX81)>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AssemblerZx81 => "Assembler (ZX81)",
    versions: [AssemblerZx81 => "Assembler (ZX81)"],
    extensions: [],
}

const ASSEMBLER_ZX81: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in Assembler for the ZX81 (Zilog Z80)

          CALL SPRINT
          DEFM ",
    output_for_hello_world!(),
    r".
          DEFB FF
          RET
SPRINT    POP HL
          LD A,(HL)
          INC HL
          PUSH HL
          CP FF
          RET Z
          CALL PRINT
          JR SPRINT"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZx81,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZx81,
            syntax: ASSEMBLER_ZX81,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ZX81,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" in Assembler for the ZX81 (Zilog Z80)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

          ",
                        ),
                        (TokenKind::Keyword, r"CALL"),
                        (
                            TokenKind::Plain,
                            r" SPRINT
          ",
                        ),
                        (TokenKind::Keyword, r"DEFM"),
                        (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
          ",
                        ),
                        (TokenKind::Keyword, r"DEFB"),
                        (
                            TokenKind::Plain,
                            r" FF
          ",
                        ),
                        (TokenKind::Keyword, r"RET"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"SPRINT"),
                        (
                            TokenKind::Plain,
                            r"    POP HL
          ",
                        ),
                        (TokenKind::Keyword, r"LD"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r",("),
                        (TokenKind::Plain, r"HL"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
          ",
                        ),
                        (TokenKind::Keyword, r"INC"),
                        (
                            TokenKind::Plain,
                            r" HL
          ",
                        ),
                        (TokenKind::Keyword, r"PUSH"),
                        (
                            TokenKind::Plain,
                            r" HL
          ",
                        ),
                        (TokenKind::Keyword, r"CP"),
                        (
                            TokenKind::Plain,
                            r" FF
          ",
                        ),
                        (TokenKind::Keyword, r"RET"),
                        (
                            TokenKind::Plain,
                            r" Z
          ",
                        ),
                        (TokenKind::Keyword, r"CALL"),
                        (
                            TokenKind::Plain,
                            r" PRINT
          ",
                        ),
                        (TokenKind::Keyword, r"JR"),
                        (TokenKind::Plain, r" SPRINT"),
                    ],
                )
            },
        }],
    };
