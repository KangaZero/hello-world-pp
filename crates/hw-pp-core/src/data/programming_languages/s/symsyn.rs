//! Hello-world snippet for `ProgrammingLanguageName::Symsyn`.
//!
//! Imported from the Hello World Collection (`files/Symsyn.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Symsyn.htm>
//! - <http://helloworldcollection.de/#Symsyn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Symsyn => "Symsyn",
    versions: [Symsyn => "Symsyn"],
    extensions: [],
}

const SYMSYN: &str = concat!(
    r"| ",
    output_for_hello_world!(),
    r" in Symsyn

 '",
    output_for_hello_world!(),
    r"' []"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Symsyn,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Symsyn,
            syntax: SYMSYN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SYMSYN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"| ",
                            output_for_hello_world!(),
                            r" in Symsyn

 '",
                            output_for_hello_world!(),
                            r"' []"
                        ),
                    )],
                )
            },
        }],
    };
