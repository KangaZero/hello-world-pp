//! Hello-world snippet for `ProgrammingLanguageName::Umka`.
//!
//! Imported from leachim6/hello-world (`u/Umka.um` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Umka.um>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Umka => "Umka",
    versions: [Umka => "Umka"],
    extensions: [Um => "um"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Umka,
        extensions: &[FileExtension::Um],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Umka,
            syntax: concat!(
                r#"fn main() {
    printf(""#,
                output_for_hello_world!(),
                r#"\n")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fn main() {
    printf(""#,
                    output_for_hello_world!(),
                    r#"\n")
}"#
                ),
            )],
        }],
    };
