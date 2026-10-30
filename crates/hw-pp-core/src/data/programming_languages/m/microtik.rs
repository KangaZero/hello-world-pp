//! Hello-world snippet for `ProgrammingLanguageName::Microtik`.
//!
//! Imported from the Hello World Collection (`files/Microtik.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Microtik.txt>
//! - <http://helloworldcollection.de/#Microtik>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Microtik => "Microtik",
    versions: [Microtik => "Microtik"],
    extensions: [],
}

const MICROTIK: &str = concat!(
    r"#",
    output_for_hello_world!(),
    r#" in Mikrotik RouterOS Scripting Host; :put (""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Microtik,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Microtik,
            syntax: MICROTIK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MICROTIK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"#",
                            output_for_hello_world!(),
                            r#" in Mikrotik RouterOS Scripting Host; :put (""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };
