//! Hello-world snippet for `ProgrammingLanguageName::Tengo`.
//!
//! Imported from leachim6/hello-world (`t/Tengo.tengo` at `a152253`),
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
    language: Tengo => "Tengo",
    versions: [Tengo => "Tengo"],
    extensions: [Tengo => "tengo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tengo,
        extensions: &[FileExtension::Tengo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tengo,
            syntax: concat!(
                r#"fmt := import("fmt")
fmt.print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fmt := import("fmt")
fmt.print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
