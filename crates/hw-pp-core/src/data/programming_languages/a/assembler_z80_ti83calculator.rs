//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Ti83calculator`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 TI83calculator.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Z80%20TI83calculator.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ80Ti83calculator => "Assembler Z80 TI83calculator",
    versions: [AssemblerZ80Ti83calculator => "Assembler Z80 TI83calculator"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Ti83calculator,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Ti83calculator,
            syntax: concat!(
                r#".NOLIST
#define   EQU   .equ
#define   equ   .equ
#define   END   .end
#define   end   .end
#include "ti83plus.inc"
.LIST
     .org 9D93h
      .db $BB,$6D
      ld a,0
      ld (CURCOL),a
      ld (CURROW),a
      ld hl,text
      B_CALL(_PutS)
      B_CALL(_NewLine)
      ret
text:
      .db ""#,
                output_for_hello_world!(),
                r#"",0
.end
end"#
            ),
            tokenized_syntax: &[
                (TokenKind::Macro, r".NOLIST"),
                (
                    TokenKind::Plain,
                    r"
#define   ",
                ),
                (TokenKind::Keyword, r"EQU"),
                (TokenKind::Plain, r"   "),
                (TokenKind::Macro, r".equ"),
                (
                    TokenKind::Plain,
                    r"
#define   ",
                ),
                (TokenKind::Keyword, r"equ"),
                (TokenKind::Plain, r"   "),
                (TokenKind::Macro, r".equ"),
                (
                    TokenKind::Plain,
                    r"
#define   END   ",
                ),
                (TokenKind::Macro, r".end"),
                (
                    TokenKind::Plain,
                    r"
#define   end   ",
                ),
                (TokenKind::Macro, r".end"),
                (
                    TokenKind::Plain,
                    r"
#include ",
                ),
                (TokenKind::String, r#""ti83plus.inc""#),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Macro, r".LIST"),
                (
                    TokenKind::Plain,
                    r"
     ",
                ),
                (TokenKind::Macro, r".org"),
                (
                    TokenKind::Plain,
                    r" 9D93h
      ",
                ),
                (TokenKind::Macro, r".db"),
                (TokenKind::Plain, r" $BB"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"$6D
      ",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" a"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"CURCOL"),
                (TokenKind::Punctuation, r"),"),
                (
                    TokenKind::Plain,
                    r"a
      ",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"CURROW"),
                (TokenKind::Punctuation, r"),"),
                (
                    TokenKind::Plain,
                    r"a
      ",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" hl"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Keyword, r"text"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Keyword, r"B_CALL"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"_PutS"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Keyword, r"B_CALL"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"_NewLine"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Keyword, r"ret"),
                (
                    TokenKind::Plain,
                    r"
text",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Macro, r".db"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Macro, r".end"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"end"),
            ],
        }],
    };
