//! Hello-world snippet for `ProgrammingLanguageName::Kumir`.
//!
//! Imported from leachim6/hello-world (`k/Kumir.kum` at `a152253`),
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
    language: Kumir => "Kumir",
    versions: [Kumir => "Kumir"],
    extensions: [Kum => "kum"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kumir,
        extensions: &[FileExtension::Kum],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kumir,
            syntax: concat!(
                r#"алг 
нач
    вывод ""#,
                output_for_hello_world!(),
                r#""
кон"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"алг 
нач
    вывод ""#,
                    output_for_hello_world!(),
                    r#""
кон"#
                ),
            )],
        }],
    };
