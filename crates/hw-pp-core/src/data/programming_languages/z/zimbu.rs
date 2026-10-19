//! Hello-world snippet for `ProgrammingLanguageName::Zimbu`.
//!
//! Imported from leachim6/hello-world (`z/Zimbu.zu` at `a152253`),
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
    language: Zimbu => "Zimbu",
    versions: [Zimbu => "Zimbu"],
    extensions: [Zu => "zu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zimbu,
        extensions: &[FileExtension::Zu],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zimbu,
            syntax: concat!(
                r#"FUNC Main() int
  IO.write(""#,
                output_for_hello_world!(),
                r#"\n")
  RETURN 0
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"FUNC Main() int
  IO.write(""#,
                    output_for_hello_world!(),
                    r#"\n")
  RETURN 0
}"#
                ),
            )],
        }],
    };
