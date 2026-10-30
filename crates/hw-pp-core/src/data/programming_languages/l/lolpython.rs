//! Hello-world snippet for `ProgrammingLanguageName::Lolpython`.
//!
//! Imported from leachim6/hello-world (`l/LOLPython.lol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LOLPython.lol>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lolpython => "LOLPython",
    versions: [Lolpython => "LOLPython"],
    extensions: [],
}

const LOLPYTHON: &str = concat!(r#"VISIBLE ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolpython,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lolpython,
            syntax: LOLPYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOLPYTHON,
                    [(
                        TokenKind::Plain,
                        concat!(r#"VISIBLE ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
