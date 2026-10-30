//! Hello-world snippet for `ProgrammingLanguageName::Zhpy`.
//!
//! Imported from leachim6/hello-world (`z/Zhpy.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zhpy.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Zhpy => "Zhpy",
    versions: [Zhpy => "Zhpy"],
    extensions: [],
}

const ZHPY: &str = concat!(r"印出 '", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zhpy,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zhpy,
            syntax: ZHPY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZHPY,
                    [(
                        TokenKind::Plain,
                        concat!(r"印出 '", output_for_hello_world!(), r"'"),
                    )],
                )
            },
        }],
    };
