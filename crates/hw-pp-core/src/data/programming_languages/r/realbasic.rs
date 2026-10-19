//! Hello-world snippet for `ProgrammingLanguageName::Realbasic`.
//!
//! Imported from leachim6/hello-world (`r/RealBasic.realbasic` at `a152253`),
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
    language: Realbasic => "RealBasic",
    versions: [Realbasic => "RealBasic"],
    extensions: [Realbasic => "realbasic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Realbasic,
        extensions: &[FileExtension::Realbasic],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Realbasic,
            syntax: concat!(
                r#"Function Run(args() as String) As Integer
  Print ""#,
                output_for_hello_world!(),
                r#""
  Quit
End Function"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Function Run(args() as String) As Integer
  Print ""#,
                    output_for_hello_world!(),
                    r#""
  Quit
End Function"#
                ),
            )],
        }],
    };
