//! Hello-world snippet for `ProgrammingLanguageName::CaEasytrievePlus`.
//!
//! Imported from the Hello World Collection (`files/CA-Easytrieve&nbsp;Plus.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CA-Easytrieve%26nbsp%3BPlus.htm>
//! - <http://helloworldcollection.de/#CA-Easytrieve%C2%A0Plus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CaEasytrievePlus => "CA-Easytrieve Plus",
    versions: [CaEasytrievePlus => "CA-Easytrieve Plus"],
    extensions: [],
}

const CA_EASYTRIEVE_PLUS: &str = concat!(
    r"* ",
    output_for_hello_world!(),
    r" in CA-Easytrieve Plus

JOB
    DISPLAY '",
    output_for_hello_world!(),
    r"'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CaEasytrievePlus,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CaEasytrievePlus,
            syntax: CA_EASYTRIEVE_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CA_EASYTRIEVE_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"* ",
                            output_for_hello_world!(),
                            r" in CA-Easytrieve Plus

JOB
    DISPLAY '",
                            output_for_hello_world!(),
                            r"'"
                        ),
                    )],
                )
            },
        }],
    };
