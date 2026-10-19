//! Hello-world snippet for `ProgrammingLanguageName::Quickbasic`.
//!
//! Imported from the Hello World Collection (`files/QuickBASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/QuickBASIC.txt>
//! - <http://helloworldcollection.de/#QuickBASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Quickbasic => "QuickBASIC",
    versions: [Quickbasic => "QuickBASIC"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quickbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quickbasic,
            syntax: concat!(
                r"REM ",
                output_for_hello_world!(),
                r#" in QuickBASIC
PRINT ""#,
                output_for_hello_world!(),
                r#""
END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"REM ",
                    output_for_hello_world!(),
                    r#" in QuickBASIC
PRINT ""#,
                    output_for_hello_world!(),
                    r#""
END"#
                ),
            )],
        }],
    };
