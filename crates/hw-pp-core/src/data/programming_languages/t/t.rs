//! Hello-world snippet for `ProgrammingLanguageName::T`.
//!
//! Imported from the Hello World Collection (`files/T.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/T.htm>
//! - <http://helloworldcollection.de/#T>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: T => "T",
    versions: [T => "T"],
    extensions: [],
}

const T: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in T

""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::T,
        extensions: &[],
        categories: &[],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::T,
            syntax: T,
            tokenized_syntax: &const {
                checked_token_lengths(
                    T,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in T

""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
