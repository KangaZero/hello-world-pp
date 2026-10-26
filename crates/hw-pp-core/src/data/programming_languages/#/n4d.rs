//! Hello-world snippet for `ProgrammingLanguageName::N4d`.
//!
//! Imported from the Hello World Collection (`files/4D.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/4D.htm>
//! - <http://helloworldcollection.de/#4D>
//! - <https://www.wikidata.org/wiki/Q238349> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N4d => "4D",
    versions: [N4d => "4D"],
    extensions: [],
}

const N4D: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in 4D, formerly known as 4th Dimension

ALERT(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4d,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4d,
            syntax: N4D,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N4D,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in 4D, formerly known as 4th Dimension

ALERT(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
