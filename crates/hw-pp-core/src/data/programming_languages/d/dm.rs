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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dm => "DM",
    versions: [Dm => "DM"],
    extensions: [Dm => "dm"],
}

const DM: &str = concat!(
    r#"mob
   Login()
      world << ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dm,
        extensions: &[FileExtension::Dm],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dm,
            syntax: DM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"mob
   Login()
      world << ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
