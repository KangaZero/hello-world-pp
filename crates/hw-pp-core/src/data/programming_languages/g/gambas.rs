//! Hello-world snippet for `ProgrammingLanguageName::Gambas`.
//!
//! Imported from the Hello World Collection (`files/Gambas.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gambas.txt>
//! - <http://helloworldcollection.de/#Gambas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gambas => "Gambas",
    versions: [Gambas => "Gambas"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gambas,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gambas,
            syntax: concat!(
                r"'************************************
' ",
                output_for_hello_world!(),
                r#" in Gambas
'************************************
PUBLIC SUB Main()

   PRINT ""#,
                output_for_hello_world!(),
                r#""

END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"'************************************
' ",
                    output_for_hello_world!(),
                    r#" in Gambas
'************************************
PUBLIC SUB Main()

   PRINT ""#,
                    output_for_hello_world!(),
                    r#""

END"#
                ),
            )],
        }],
    };
