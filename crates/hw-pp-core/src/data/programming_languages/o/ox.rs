//! Hello-world snippet for `ProgrammingLanguageName::Ox`.
//!
//! Imported from leachim6/hello-world (`o/OX.oz` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/OX.oz>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scientific, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ox => "OX",
    versions: [Ox => "OX"],
    extensions: [],
}

const OX: &str = concat!(r"{Show '", output_for_hello_world!(), r"'}");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ox,
        extensions: &[],
        categories: &[Scientific],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ox,
            syntax: OX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OX,
                    [(
                        TokenKind::Plain,
                        concat!(r"{Show '", output_for_hello_world!(), r"'}"),
                    )],
                )
            },
        }],
    };
