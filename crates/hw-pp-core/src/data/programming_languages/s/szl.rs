//! Hello-world snippet for `ProgrammingLanguageName::Szl`.
//!
//! Imported from the Hello World Collection (`files/szl.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/szl.htm>
//! - <http://helloworldcollection.de/#szl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Szl => "szl",
    versions: [Szl => "szl"],
    extensions: [],
}

const SZL: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r" in szl

$global msg {",
    output_for_hello_world!(),
    r"}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Szl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Szl,
            syntax: SZL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SZL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r" in szl

$global msg {",
                            output_for_hello_world!(),
                            r"}"
                        ),
                    )],
                )
            },
        }],
    };
