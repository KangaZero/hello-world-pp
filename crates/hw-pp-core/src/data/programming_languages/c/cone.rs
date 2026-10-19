//! Hello-world snippet for `ProgrammingLanguageName::Cone`.
//!
//! Imported from leachim6/hello-world (`c/Cone.cone` at `a152253`),
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
    language: Cone => "Cone",
    versions: [Cone => "Cone"],
    extensions: [Cone => "cone"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cone,
        extensions: &[FileExtension::Cone],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cone,
            syntax: concat!(
                r#"import stdio::*

fn main():
  print <- ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import stdio::*

fn main():
  print <- ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
