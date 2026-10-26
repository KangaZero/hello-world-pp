//! Hello-world snippet for `ProgrammingLanguageName::Omnimark`.
//!
//! Imported from the Hello World Collection (`files/Omnimark.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Omnimark.txt>
//! - <http://helloworldcollection.de/#Omnimark>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Omnimark => "Omnimark",
    versions: [Omnimark => "Omnimark"],
    extensions: [],
}

const OMNIMARK: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Omnimark


process
     output ""#,
    output_for_hello_world!(),
    r#"%n""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Omnimark,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Omnimark,
            syntax: OMNIMARK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OMNIMARK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in Omnimark


process
     output ""#,
                            output_for_hello_world!(),
                            r#"%n""#
                        ),
                    )],
                )
            },
        }],
    };
