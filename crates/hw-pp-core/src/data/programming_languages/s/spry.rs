//! Hello-world snippet for `ProgrammingLanguageName::Spry`.
//!
//! Imported from the Hello World Collection (`files/Spry.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Spry.htm>
//! - <http://helloworldcollection.de/#Spry>
//! - <https://github.com/gokr/spry> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Spry => "Spry",
    versions: [Spry => "Spry"],
    extensions: [],
}

const SPRY: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Spry

echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spry,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spry,
            syntax: SPRY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPRY,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in Spry

echo ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
