//! Hello-world snippet for `ProgrammingLanguageName::Scss`.
//!
//! Imported from leachim6/hello-world (`s/SCSS.scss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SCSS.scss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Scss => "SCSS",
    versions: [Scss => "SCSS"],
    extensions: [Scss => "scss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scss,
        extensions: &[FileExtension::Scss],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scss,
            syntax: concat!(
                r#"body::before {
    content: ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"body::before {
    content: ""#,
                    output_for_hello_world!(),
                    r#"";
}"#
                ),
            )],
        }],
    };
