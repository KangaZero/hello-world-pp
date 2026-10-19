//! Hello-world snippet for `ProgrammingLanguageName::Peregrine`.
//!
//! Imported from leachim6/hello-world (`p/Peregrine.pe` at `a152253`),
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
    language: Peregrine => "Peregrine",
    versions: [Peregrine => "Peregrine"],
    extensions: [Pe => "pe"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Peregrine,
        extensions: &[FileExtension::Pe],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Peregrine,
            syntax: concat!(
                r#"def main():
  printf(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"def main():
  printf(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
