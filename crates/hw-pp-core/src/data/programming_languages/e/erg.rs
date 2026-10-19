//! Hello-world snippet for `ProgrammingLanguageName::Erg`.
//!
//! Imported from leachim6/hello-world (`e/Erg.er` at `a152253`),
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
    language: Erg => "Erg",
    versions: [Erg => "Erg"],
    extensions: [Er => "er"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erg,
        extensions: &[FileExtension::Er],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Erg,
            syntax: concat!(r#"print! ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print! ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
