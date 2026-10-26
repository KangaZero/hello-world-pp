//! Hello-world snippet for `ProgrammingLanguageName::Tacl`.
//!
//! Imported from the Hello World Collection (`files/TACL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TACL.htm>
//! - <http://helloworldcollection.de/#TACL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tacl => "TACL",
    versions: [Tacl => "TACL"],
    extensions: [],
}

const TACL: &str = concat!(
    r"Comment -- ",
    output_for_hello_world!(),
    r" for TACL (Tandem Advanced Command Language)

?tacl macro

#OUTPUT ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tacl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tacl,
            syntax: TACL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TACL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Comment -- ",
                            output_for_hello_world!(),
                            r" for TACL (Tandem Advanced Command Language)

?tacl macro

#OUTPUT ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
