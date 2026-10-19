//! Hello-world snippet for `ProgrammingLanguageName::Wml`.
//!
//! Imported from leachim6/hello-world (`w/WML.wml` at `a152253`),
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
    language: Wml => "WML",
    versions: [Wml => "WML"],
    extensions: [Wml => "wml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wml,
        extensions: &[FileExtension::Wml],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wml,
            syntax: concat!(
                r#"#define HELLOWORLD
  [message]
    speaker="narrator"
    message=_""#,
                output_for_hello_world!(),
                r#""
  [/message]
#enddef"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#define HELLOWORLD
  [message]
    speaker="narrator"
    message=_""#,
                    output_for_hello_world!(),
                    r#""
  [/message]
#enddef"#
                ),
            )],
        }],
    };
