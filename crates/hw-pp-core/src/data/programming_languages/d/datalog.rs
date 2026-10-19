//! Hello-world snippet for `ProgrammingLanguageName::Datalog`.
//!
//! Imported from leachim6/hello-world (PR #1524 `d/Datalog.dl`),
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
    language: Datalog => "Datalog",
    versions: [Datalog => "Datalog"],
    extensions: [Dl => "dl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Datalog,
        extensions: &[FileExtension::Dl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Datalog,
            syntax: concat!(
                r#"output(""#,
                output_for_hello_world!(),
                r#"") :- greeting(true).
greeting(true)."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"output(""#,
                    output_for_hello_world!(),
                    r#"") :- greeting(true).
greeting(true)."#
                ),
            )],
        }],
    };
