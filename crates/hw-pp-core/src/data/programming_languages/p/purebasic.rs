//! Hello-world snippet for `ProgrammingLanguageName::Purebasic`.
//!
//! Imported from leachim6/hello-world (`p/PureBasic.pb` at `a152253`),
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
    language: Purebasic => "PureBasic",
    versions: [Purebasic => "PureBasic"],
    extensions: [Pb => "pb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purebasic,
        extensions: &[FileExtension::Pb],
        categories: &[],
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
