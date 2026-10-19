//! Hello-world snippet for `ProgrammingLanguageName::N4thDimension`.
//!
//! Imported from leachim6/hello-world (`#/4th Dimension.4dd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/4th%20Dimension.4dd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: N4thDimension => "4th Dimension",
    versions: [N4thDimension => "4th Dimension"],
    extensions: [E4dd => "4dd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4thDimension,
        extensions: &[FileExtension::E4dd],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4thDimension,
            syntax: concat!(
                r#"OPEN WINDOW (10;45;500;330;0;"Hello Window")
While (True)
  MESSAGE (""#,
                output_for_hello_world!(),
                r#"")
End while"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"OPEN WINDOW (10;45;500;330;0;"Hello Window")
While (True)
  MESSAGE (""#,
                    output_for_hello_world!(),
                    r#"")
End while"#
                ),
            )],
        }],
    };
