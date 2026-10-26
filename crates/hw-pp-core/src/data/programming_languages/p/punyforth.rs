//! Hello-world snippet for `ProgrammingLanguageName::Punyforth`.
//!
//! Imported from the Hello World Collection (`files/Punyforth.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Punyforth.htm>
//! - <http://helloworldcollection.de/#Punyforth>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Punyforth => "Punyforth",
    versions: [Punyforth => "Punyforth"],
    extensions: [],
}

const PUNYFORTH: &str = concat!(
    r"\ ",
    output_for_hello_world!(),
    r#" in Punyforth

println: ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Punyforth,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Punyforth,
            syntax: PUNYFORTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PUNYFORTH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"\ ",
                            output_for_hello_world!(),
                            r#" in Punyforth

println: ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
