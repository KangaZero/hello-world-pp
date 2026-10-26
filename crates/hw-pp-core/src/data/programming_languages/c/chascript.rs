//! Hello-world snippet for `ProgrammingLanguageName::Chascript`.
//!
//! Imported from leachim6/hello-world (`c/ChaScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/ChaScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Chascript => "ChaScript",
    versions: [Chascript => "ChaScript"],
    extensions: [],
}

const CHASCRIPT: &str = concat!(r"দেখাও(”", output_for_hello_world!(), r"”)");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chascript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chascript,
            syntax: CHASCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHASCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r"দেখাও(”", output_for_hello_world!(), r"”)"),
                    )],
                )
            },
        }],
    };
