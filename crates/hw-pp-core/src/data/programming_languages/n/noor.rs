//! Hello-world snippet for `ProgrammingLanguageName::Noor`.
//!
//! Imported from the Hello World Collection (`files/Noor.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Noor.htm>
//! - <http://helloworldcollection.de/#Noor>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Noor => "Noor",
    versions: [Noor => "Noor"],
    extensions: [],
}

const NOOR: &str = concat!(r#"اطبع ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Noor,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Noor,
            syntax: NOOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NOOR,
                    [(
                        TokenKind::Plain,
                        concat!(r#"اطبع ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
