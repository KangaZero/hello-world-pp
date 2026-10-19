//! Hello-world snippet for `ProgrammingLanguageName::Pakhi`.
//!
//! Imported from leachim6/hello-world (`p/Pakhi.pakhi` at `a152253`),
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
    language: Pakhi => "Pakhi",
    versions: [Pakhi => "Pakhi"],
    extensions: [Pakhi => "pakhi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pakhi,
        extensions: &[FileExtension::Pakhi],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pakhi,
            syntax: concat!(r#"দেখাও ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"দেখাও ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
