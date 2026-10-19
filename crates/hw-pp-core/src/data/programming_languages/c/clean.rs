//! Hello-world snippet for `ProgrammingLanguageName::Clean`.
//!
//! Imported from leachim6/hello-world (`c/Clean.icl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Clean.icl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clean => "Clean",
    versions: [Clean => "Clean"],
    extensions: [Icl => "icl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clean,
        extensions: &[FileExtension::Icl],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clean,
            syntax: concat!(
                r#"module hello
Start :: {#Char}
Start = ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module hello
Start :: {#Char}
Start = ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
