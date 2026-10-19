//! Hello-world snippet for `ProgrammingLanguageName::Iscom`.
//!
//! Imported from leachim6/hello-world (`i/Iscom.iscom` at `a152253`),
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
    language: Iscom => "Iscom",
    versions: [Iscom => "Iscom"],
    extensions: [Iscom => "iscom"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Iscom,
        extensions: &[FileExtension::Iscom],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Iscom,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"" @=10"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"" @=10"#),
            )],
        }],
    };
