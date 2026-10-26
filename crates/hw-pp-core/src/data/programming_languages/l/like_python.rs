//! Hello-world snippet for `ProgrammingLanguageName::LikePython`.
//!
//! Imported from leachim6/hello-world (`l/Like, Python.lp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Like%2C%20Python.lp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LikePython => "Like, Python",
    versions: [LikePython => "Like, Python"],
    extensions: [Lp => "lp"],
}

const LIKE_PYTHON: &str = concat!(
    r#"#!usr/bin/python

yo just print like ""#,
    output_for_hello_world!(),
    r#"" bro"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LikePython,
        extensions: &[FileExtension::Lp],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LikePython,
            syntax: LIKE_PYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIKE_PYTHON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"#!usr/bin/python

yo just print like ""#,
                            output_for_hello_world!(),
                            r#"" bro"#
                        ),
                    )],
                )
            },
        }],
    };
