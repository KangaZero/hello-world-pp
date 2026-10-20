//! Hello-world snippet for `ProgrammingLanguageName::Kumir`.
//!
//! Imported from leachim6/hello-world (`k/Kumir.kum` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kumir.kum>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Educational],
        families: &[],
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
