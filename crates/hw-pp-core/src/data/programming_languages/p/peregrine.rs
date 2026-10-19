//! Hello-world snippet for `ProgrammingLanguageName::Peregrine`.
//!
//! Imported from leachim6/hello-world (`p/Peregrine.pe` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Peregrine.pe>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Compiled],
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
