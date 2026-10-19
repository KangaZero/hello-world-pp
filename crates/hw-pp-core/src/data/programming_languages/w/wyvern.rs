//! Hello-world snippet for `ProgrammingLanguageName::Wyvern`.
//!
//! Imported from leachim6/hello-world (`w/Wyvern.wyv` at `a152253`),
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
    language: Wyvern => "Wyvern",
    versions: [Wyvern => "Wyvern"],
    extensions: [Wyv => "wyv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wyvern,
        extensions: &[FileExtension::Wyv],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wyvern,
            syntax: concat!(
                r#"require stdout

stdout.print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"require stdout

stdout.print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
