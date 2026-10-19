//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Ti83calculator`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 TI83calculator.asm` at `a152253`),
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
    language: AssemblerZ80Ti83calculator => "Assembler Z80 TI83calculator",
    versions: [AssemblerZ80Ti83calculator => "Assembler Z80 TI83calculator"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Ti83calculator,
        extensions: &[],
        categories: &[],
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
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
            )],
        }],
    };
