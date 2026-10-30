//! Hello-world snippet for `ProgrammingLanguageName::Dos`.
//!
//! Imported from leachim6/hello-world (`d/dos.bat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/dos.bat>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dos => "dos",
    versions: [Dos => "dos"],
    extensions: [],
}

const DOS: &str = concat!(r"@echo ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dos,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dos,
            syntax: DOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DOS,
                    [
                        (TokenKind::Keyword, r"@echo"),
                        (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
                    ],
                )
            },
        }],
    };
