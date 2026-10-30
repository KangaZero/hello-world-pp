//! Hello-world snippet for `ProgrammingLanguageName::OplSimple`.
//!
//! Imported from the Hello World Collection (`files/OPL.simple` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/OPL.simple>
//! - <http://helloworldcollection.de/#OPL.simple>
//! - <https://en.wikipedia.org/wiki/Open_Programming_Language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: OplSimple => "OPL.simple",
    versions: [OplSimple => "OPL.simple"],
    extensions: [],
}

const OPL_SIMPLE: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OplSimple,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OplSimple,
            syntax: OPL_SIMPLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OPL_SIMPLE,
                    [(
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
                )
            },
        }],
    };
