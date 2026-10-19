//! Hello-world snippet for `ProgrammingLanguageName::LikePython`.
//!
//! Imported from leachim6/hello-world (`l/Like, Python.lp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: LikePython => "Like, Python",
    versions: [LikePython => "Like, Python"],
    extensions: [Lp => "lp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LikePython,
        extensions: &[FileExtension::Lp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LikePython,
            syntax: concat!(
                r#"#!usr/bin/python

yo just print like ""#,
                output_for_hello_world!(),
                r#"" bro"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!usr/bin/python

yo just print like ""#,
                    output_for_hello_world!(),
                    r#"" bro"#
                ),
            )],
        }],
    };
