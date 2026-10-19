//! Hello-world snippet for `ProgrammingLanguageName::OplSimple`.
//!
//! Imported from the Hello World Collection (`files/OPL.simple` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/OPL.simple>
//! - <http://helloworldcollection.de/#OPL.simple>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: OplSimple => "OPL.simple",
    versions: [OplSimple => "OPL.simple"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OplSimple,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OplSimple,
            syntax: concat!(
                r"REM ",
                output_for_hello_world!(),
                r#" for OPL (Psion Organizer 3a)
REM Simple version

PROC HELLO:
  PRINT ""#,
                output_for_hello_world!(),
                r#""
  GET
ENDP"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"REM ",
                    output_for_hello_world!(),
                    r#" for OPL (Psion Organizer 3a)
REM Simple version

PROC HELLO:
  PRINT ""#,
                    output_for_hello_world!(),
                    r#""
  GET
ENDP"#
                ),
            )],
        }],
    };
