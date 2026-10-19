//! Hello-world snippet for `ProgrammingLanguageName::Oberon2`.
//!
//! Imported from leachim6/hello-world (`o/Oberon 2.obn` at `a152253`),
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
    language: Oberon2 => "Oberon 2",
    versions: [Oberon2 => "Oberon 2"],
    extensions: [Obn => "obn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oberon2,
        extensions: &[FileExtension::Obn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oberon2,
            syntax: concat!(
                r#"MODULE HelloWorld;                     
IMPORT Out;
BEGIN
  Out.String( ""#,
                output_for_hello_world!(),
                r#"" );
  Out.Ln;
END HelloWorld."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"MODULE HelloWorld;                     
IMPORT Out;
BEGIN
  Out.String( ""#,
                    output_for_hello_world!(),
                    r#"" );
  Out.Ln;
END HelloWorld."#
                ),
            )],
        }],
    };
