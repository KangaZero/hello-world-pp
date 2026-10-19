//! Hello-world snippet for `ProgrammingLanguageName::Ec`.
//!
//! Imported from leachim6/hello-world (`e/Ec.ec` at `a152253`),
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
    language: Ec => "Ec",
    versions: [Ec => "Ec"],
    extensions: [Ec => "ec"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ec,
        extensions: &[FileExtension::Ec],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ec,
            syntax: concat!(
                r#"class HelloWorldApp : Application
{
   void Main()
   {
      PrintLn(""#,
                output_for_hello_world!(),
                r#"");
   }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class HelloWorldApp : Application
{
   void Main()
   {
      PrintLn(""#,
                    output_for_hello_world!(),
                    r#"");
   }
}"#
                ),
            )],
        }],
    };
