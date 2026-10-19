//! Hello-world snippet for `ProgrammingLanguageName::Cotowali`.
//!
//! Imported from leachim6/hello-world (`c/Cotowali.li` at `a152253`),
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
    language: Cotowali => "Cotowali",
    versions: [Cotowali => "Cotowali"],
    extensions: [Li => "li"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cotowali,
        extensions: &[FileExtension::Li],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cotowali,
            syntax: concat!(
                r#"fn helloworld(): string {
  return ""#,
                output_for_hello_world!(),
                r#""
}

helloworld()"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fn helloworld(): string {
  return ""#,
                    output_for_hello_world!(),
                    r#""
}

helloworld()"#
                ),
            )],
        }],
    };
