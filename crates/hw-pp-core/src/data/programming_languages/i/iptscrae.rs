//! Hello-world snippet for `ProgrammingLanguageName::Iptscrae`.
//!
//! Imported from the Hello World Collection (`files/Iptscrae.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Iptscrae.txt>
//! - <http://helloworldcollection.de/#Iptscrae>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Iptscrae => "Iptscrae",
    versions: [Iptscrae => "Iptscrae"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Iptscrae,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Iptscrae,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in Iptscrae.
; 1. from the chat prompt:

/ ""#,
                output_for_hello_world!(),
                r#"" SAY

; 2. in a cyborg:

ON OUTCHAT {
    {  ""#,
                output_for_hello_world!(),
                r#"" SAY
    } CHATSTR "say it" == IF
}

; 3. in a room script:

ON SELECT {
    ""#,
                output_for_hello_world!(),
                r#"" SAY
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in Iptscrae.
; 1. from the chat prompt:

/ ""#,
                    output_for_hello_world!(),
                    r#"" SAY

; 2. in a cyborg:

ON OUTCHAT {
    {  ""#,
                    output_for_hello_world!(),
                    r#"" SAY
    } CHATSTR "say it" == IF
}

; 3. in a room script:

ON SELECT {
    ""#,
                    output_for_hello_world!(),
                    r#"" SAY
}"#
                ),
            )],
        }],
    };
