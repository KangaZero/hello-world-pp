//! Hello-world snippet for `ProgrammingLanguageName::Datalog`.
//!
//! Imported from leachim6/hello-world (PR #1524 `d/Datalog.dl`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1524>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Datalog => "Datalog",
    versions: [Datalog => "Datalog"],
    extensions: [Dl => "dl"],
}

const DATALOG: &str = concat!(
    r#"output(""#,
    output_for_hello_world!(),
    r#"") :- greeting(true).
greeting(true)."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Datalog,
        extensions: &[FileExtension::Dl],
        categories: &[Query],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Datalog,
            syntax: DATALOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DATALOG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"output(""#,
                            output_for_hello_world!(),
                            r#"") :- greeting(true).
greeting(true)."#
                        ),
                    )],
                )
            },
        }],
    };
