//! Hello-world snippet for `ProgrammingLanguageName::Cotowali`.
//!
//! Imported from leachim6/hello-world (`c/Cotowali.li` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cotowali.li>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
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
