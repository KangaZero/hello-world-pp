//! Hello-world snippet for `ProgrammingLanguageName::Purebasic`.
//!
//! Imported from leachim6/hello-world (`p/PureBasic.pb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PureBasic.pb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Purebasic => "PureBasic",
    versions: [Purebasic => "PureBasic"],
    extensions: [Pb => "pb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purebasic,
        extensions: &[FileExtension::Pb],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Purebasic,
            syntax: concat!(
                r#"If OpenConsole()
  PrintN(""#,
                output_for_hello_world!(),
                r#"")
EndIf"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"If OpenConsole()
  PrintN(""#,
                    output_for_hello_world!(),
                    r#"")
EndIf"#
                ),
            )],
        }],
    };
