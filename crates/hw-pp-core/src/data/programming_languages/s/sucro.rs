//! Hello-world snippet for `ProgrammingLanguageName::Sucro`.
//!
//! Imported from the Hello World Collection (`files/Sucro.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Sucro.htm>
//! - <http://helloworldcollection.de/#Sucro>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sucro => "Sucro",
    versions: [Sucro => "Sucro"],
    extensions: [],
}

const SUCRO: &str = concat!(
    output_for_hello_world!(),
    r" in Sucro
{R}",
    output_for_hello_world!(),
    r"{end}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sucro,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sucro,
            syntax: SUCRO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SUCRO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r" in Sucro
{R}",
                            output_for_hello_world!(),
                            r"{end}"
                        ),
                    )],
                )
            },
        }],
    };
