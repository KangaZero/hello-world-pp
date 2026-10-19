//! Hello-world snippet for `ProgrammingLanguageName::Jaksel`.
//!
//! Imported from leachim6/hello-world (PR #1638 `j/Jaksel.jaksel`),
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
    language: Jaksel => "Jaksel",
    versions: [Jaksel => "Jaksel"],
    extensions: [Jaksel => "jaksel"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jaksel,
        extensions: &[FileExtension::Jaksel],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jaksel,
            syntax: concat!(r#"spill ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"spill ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
