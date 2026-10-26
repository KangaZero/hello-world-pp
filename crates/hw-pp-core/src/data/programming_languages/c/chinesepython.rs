//! Hello-world snippet for `ProgrammingLanguageName::Chinesepython`.
//!
//! Imported from leachim6/hello-world (`c/ChinesePython.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/ChinesePython.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Chinesepython => "ChinesePython",
    versions: [Chinesepython => "ChinesePython"],
    extensions: [],
}

const CHINESEPYTHON: &str = concat!(r#"寫 ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chinesepython,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chinesepython,
            syntax: CHINESEPYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHINESEPYTHON,
                    [(
                        TokenKind::Plain,
                        concat!(r#"寫 ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
