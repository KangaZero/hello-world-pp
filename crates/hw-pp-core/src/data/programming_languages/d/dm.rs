//! Hello-world snippet for `ProgrammingLanguageName::Dm`.
//!
//! Imported from leachim6/hello-world (PR #1481 `d/DM.dm`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1481>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dm => "DM",
    versions: [Dm => "DM"],
    extensions: [Dm => "dm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dm,
        extensions: &[FileExtension::Dm],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dm,
            syntax: concat!(
                r#"mob
   Login()
      world << ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"mob
   Login()
      world << ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
