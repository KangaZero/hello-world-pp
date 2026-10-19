//! Hello-world snippet for `ProgrammingLanguageName::Macro10`.
//!
//! Imported from the Hello World Collection (`files/MACRO-10.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MACRO-10.txt>
//! - <http://helloworldcollection.de/#MACRO-10>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Macro10 => "MACRO-10",
    versions: [Macro10 => "MACRO-10"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Macro10,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Macro10,
            syntax: concat!(
                r"TITLE ",
                output_for_hello_world!(),
                r"
; ",
                output_for_hello_world!(),
                r" IN MACRO 10 FOR TOPS-10
ENTRY OUTPUT
SEARCH UUOSYM

LAB:    ASCIZ /",
                output_for_hello_world!(),
                r"
/
OUTPUT: OUTSTR LAB              ; OUTPUT MESSAGE
        MONRT.                  ; RETURN TO MONITOR          
        END OUTPUT"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"TITLE ",
                    output_for_hello_world!(),
                    r"
; ",
                    output_for_hello_world!(),
                    r" IN MACRO 10 FOR TOPS-10
ENTRY OUTPUT
SEARCH UUOSYM

LAB:    ASCIZ /",
                    output_for_hello_world!(),
                    r"
/
OUTPUT: OUTSTR LAB              ; OUTPUT MESSAGE
        MONRT.                  ; RETURN TO MONITOR          
        END OUTPUT"
                ),
            )],
        }],
    };
