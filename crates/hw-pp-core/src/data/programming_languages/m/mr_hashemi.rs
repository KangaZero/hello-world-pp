//! Hello-world snippet for `ProgrammingLanguageName::MrHashemi`.
//!
//! Imported from leachim6/hello-world (`m/Mr Hashemi.hashemi` at `a152253`),
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
    language: MrHashemi => "Mr Hashemi",
    versions: [MrHashemi => "Mr Hashemi"],
    extensions: [Hashemi => "hashemi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MrHashemi,
        extensions: &[FileExtension::Hashemi],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MrHashemi,
            syntax: concat!(
                r#"bebin azinja() {
  bechap(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"bebin azinja() {
  bechap(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
