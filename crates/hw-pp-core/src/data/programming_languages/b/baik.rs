//! Hello-world snippet for `ProgrammingLanguageName::Baik`.
//!
//! Imported from leachim6/hello-world (`b/BAIK.ina` at `a152253`),
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
    language: Baik => "BAIK",
    versions: [Baik => "BAIK"],
    extensions: [Ina => "ina"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Baik,
        extensions: &[FileExtension::Ina],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Baik,
            syntax: concat!(r#"tulis ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"tulis ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
