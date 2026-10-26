//! Hello-world snippet for `ProgrammingLanguageName::LotusNoteFormula`.
//!
//! Imported from the Hello World Collection (`files/Lotus&nbsp;Note&nbsp;Formula.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Lotus%26nbsp%3BNote%26nbsp%3BFormula.txt>
//! - <http://helloworldcollection.de/#Lotus%C2%A0Note%C2%A0Formula>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LotusNoteFormula => "Lotus Note Formula",
    versions: [LotusNoteFormula => "Lotus Note Formula"],
    extensions: [],
}

const LOTUS_NOTE_FORMULA: &str = concat!(
    r#"REM "Lotus Note Formula Language";
@Prompt([ok];"Hi there";""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LotusNoteFormula,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LotusNoteFormula,
            syntax: LOTUS_NOTE_FORMULA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOTUS_NOTE_FORMULA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"REM "Lotus Note Formula Language";
@Prompt([ok];"Hi there";""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };
