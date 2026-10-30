//! Hello-world snippet for `ProgrammingLanguageName::Tabloid`.
//!
//! Imported from leachim6/hello-world (`t/Tabloid` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tabloid>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Tabloid => "Tabloid",
    versions: [Tabloid => "Tabloid"],
    extensions: [],
}

const TABLOID: &str = concat!(
    r"YOU WON'T WANT TO MISS '",
    output_for_hello_world!(),
    r"'

PLEASE LIKE AND SUBSCRIBE"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tabloid,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tabloid,
            syntax: TABLOID,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TABLOID,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"YOU WON'T WANT TO MISS '",
                            output_for_hello_world!(),
                            r"'

PLEASE LIKE AND SUBSCRIBE"
                        ),
                    )],
                )
            },
        }],
    };
