//! Hello-world snippet for `ProgrammingLanguageName::Red`.
//!
//! Imported from leachim6/hello-world (`r/Red.red` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Red.red>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Red => "Red",
    versions: [Red => "Red"],
    extensions: [Red => "red"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Red,
        extensions: &[FileExtension::Red],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Red,
            syntax: concat!(
                r#"Red [Needs: 'View]
view [text ""#,
                output_for_hello_world!(),
                r#""] "#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Red [Needs: 'View]
view [text ""#,
                    output_for_hello_world!(),
                    r#""] "#
                ),
            )],
        }],
    };
