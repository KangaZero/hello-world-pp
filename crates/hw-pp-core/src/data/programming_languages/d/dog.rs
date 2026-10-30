//! Hello-world snippet for `ProgrammingLanguageName::Dog`.
//!
//! Imported from leachim6/hello-world (`d/DOG` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DOG>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dog => "DOG",
    versions: [Dog => "DOG"],
    extensions: [],
}

const DOG: &str = concat!(r#"bark ""#, output_for_hello_world!(), r#"\n""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dog,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dog,
            syntax: DOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DOG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"bark ""#, output_for_hello_world!(), r#"\n""#),
                    )],
                )
            },
        }],
    };
