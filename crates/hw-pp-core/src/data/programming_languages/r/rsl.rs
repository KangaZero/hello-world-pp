//! Hello-world snippet for `ProgrammingLanguageName::Rsl`.
//!
//! Imported from the Hello World Collection (`files/RSL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RSL.txt>
//! - <http://helloworldcollection.de/#RSL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rsl => "RSL",
    versions: [Rsl => "RSL"],
    extensions: [],
}

const RSL: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r" in RSL (RS-Bank Language)

[",
    output_for_hello_world!(),
    r"];"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rsl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rsl,
            syntax: RSL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RSL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in RSL (RS-Bank Language)

[",
                            output_for_hello_world!(),
                            r"];"
                        ),
                    )],
                )
            },
        }],
    };
