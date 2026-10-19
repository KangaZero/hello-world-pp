//! Hello-world snippet for `ProgrammingLanguageName::Nit`.
//!
//! Imported from leachim6/hello-world (`n/Nit.nit` at `a152253`),
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
    language: Nit => "Nit",
    versions: [Nit => "Nit"],
    extensions: [Nit => "nit"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nit,
        extensions: &[FileExtension::Nit],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nit,
            syntax: concat!(
                r#"print ""#,
                output_for_hello_world!(),
                r#""
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"print ""#,
                    output_for_hello_world!(),
                    r#""
"#
                ),
            )],
        }],
    };
