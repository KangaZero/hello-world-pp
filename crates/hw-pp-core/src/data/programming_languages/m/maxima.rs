//! Hello-world snippet for `ProgrammingLanguageName::Maxima`.
//!
//! Imported from leachim6/hello-world (`m/Maxima.max` at `a152253`),
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
    language: Maxima => "Maxima",
    versions: [Maxima => "Maxima"],
    extensions: [Max => "max"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maxima,
        extensions: &[FileExtension::Max],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maxima,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")$"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"")$"#),
            )],
        }],
    };
